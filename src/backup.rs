use log::*;
use regex::Regex;
use std::io::{BufRead, BufReader, Lines};
use std::process::{ChildStdout, Command, Stdio};

/*-------------------------------------*/

/* ProcessState */

#[derive(PartialEq)]
enum ProcessState {
    Ready,
    Started,
    Completed,
}

/*-------------------------------------*/

/* BackupConfig */

pub struct BackupConfig {
    name: String,         //arbitrary name
    from: Vec<String>,    //rsync sources
    to: String,           //rsync destination
    options: Vec<String>, //rsync options
}

impl BackupConfig {
    pub fn new(name: String, from: Vec<String>, to: String, options: Vec<String>) -> Self {
        BackupConfig {
            name,
            from,
            to,
            options,
        }
    }
}

/*-------------------------------------*/

/* Backup */
//corresponds to each rsync execution

pub struct Backup {
    config: BackupConfig,
    percentage: u8,
    buf: Option<Lines<BufReader<ChildStdout>>>, //iterator to read output real-time
    pub output: String, //The container to which each read output is appended. This field is displayed in GUI.
    running_status: ProcessState, //Is the bound rsync command now being executed or completed?
    regex: Regex,       //used to capture the percentage info from the command output
}

impl Backup {
    fn new(config: BackupConfig) -> Self {
        Backup {
            config,
            percentage: 0,
            buf: None,
            output: String::new(),
            running_status: ProcessState::Ready,
            regex: Regex::new(r"\d{1,3}%").unwrap(),
        }
    }

    fn async_run(&mut self) -> () {
        debug!("Backup::async_run()");
        assert!(self.running_status == ProcessState::Ready);
        //         let child = Command::new("./mock.sh")
        //             .args(["./mock.txt"])
        //             .stdout(Stdio::piped())
        //             .spawn()
        //             .unwrap();
        let child = Command::new("rsync")
            .args(
                ["--info=name,progress2"] //This is needed to calculate the progress.
                    .into_iter()
                    .map(|e: &str| String::from(e))
                    .chain(self.config.options.clone()) //options
                    .chain(self.config.from.clone()) //source
                    .chain(vec![self.config.to.clone()]) //dest
                    .collect::<Vec<String>>(),
            )
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        self.buf = Some(BufReader::new(child.stdout.unwrap()).lines());
        self.running_status = ProcessState::Started;
    }

    fn update_progress(&mut self) -> () {
        debug!("Backup::update_progress()");

        //TODO FIXME
        //bottleneck
        //一回の呼び出しで一行しか読まないため、バックアップ対象が多いと異常に時間がかかり、実用に耐えない(60Hzで動くeguiの限界か)
        //`for i in 1..1000`でラップすれば高速化することを確認済みだが(†1)、そうするとバックアップ対象が少ないときのリアルタイム性が損なわれる
        //†1: これはregexがボトルネックではないことも示唆している

        //reads the next available line from the command output
        match self.buf.as_mut().unwrap().next() {
            Some(line) => {
                let line: String = line.unwrap();
                //if `line` is a line which indicates the progress
                if let Some(m) = self.regex.find_iter(&line).last() {
                    let m = m.as_str();
                    self.percentage = m[..(m.len() - 1)].parse().unwrap();
                } else {
                    self.output.push_str(&line);
                    self.output.push('\n');
                    info!("{}", line);
                }
            }
            None => {
                self.percentage = 100;
                self.running_status = ProcessState::Completed;
            }
        }
    }

    pub fn get_name(&self) -> &String {
        &self.config.name
    }

    pub fn get_percentage(&self) -> u8 {
        self.percentage
    }
}

/*-------------------------------------*/

/* Backups */
//corresponds to the list of rsync commands

pub struct Backups {
    current_index: usize, //Which backup is now being processed?
    pub backup_list: Vec<Backup>,
    completed: bool, //Is the execution as a whole completed?
}

impl Backups {
    pub fn new(config_list: Vec<BackupConfig>) -> Result<Self, String> {
        if (config_list.is_empty()) {
            return Err(String::from("`config_list` is empty."));
        }
        Ok(Backups {
            current_index: 0,
            backup_list: config_list.into_iter().map(|e| Backup::new(e)).collect(),
            completed: false,
        })
    }

    pub fn get_current_backup(&mut self) -> &mut Backup {
        debug!("Backups::get_current_backup()");
        if (self.current_index >= self.backup_list.len()) {
            let last_index = self.backup_list.len() - 1;
            &mut self.backup_list[last_index]
        } else {
            &mut self.backup_list[self.current_index]
        }
    }

    pub fn get_backup_list(&self) -> &Vec<Backup> {
        &self.backup_list
    }

    //called from `update()` in GUI
    pub fn update_progress(&mut self) -> bool {
        trace!("Backups::update_progress()");
        if (self.completed) {
            return false;
        }
        let mut target: &mut Backup = self.get_current_backup();
        match target.running_status {
            ProcessState::Ready => target.async_run(),
            ProcessState::Started => (),
            ProcessState::Completed => {
                self.current_index += 1; //tries to run the next backup
                target = self.get_current_backup();
                //if there doesn't exist a next backup
                if (target.running_status == ProcessState::Completed) {
                    return false;
                }
                target.async_run();
            }
        }
        target.update_progress(); //makes the current `target` read the command output
        true
    }
}

/*-------------------------------------*/
