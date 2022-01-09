#!/usr/bin/env bash
#This script mimics the rsync output and is intended to be used for debugging or development use.

sec=0.1

echo -e 'gui/'
sleep "${sec}"
echo -e 'gui/.gitignore'
sleep "${sec}"
echo -e '              8   0%    0.00kB/s    0:00:00                8   0%    0.00kB/s    0:00:00 (xfr#1, to-chk=20/22)              8   0%    0.00kB/s    0:00:00 (xfr#1, to-chk=14/22)'
sleep "${sec}"
echo -e 'gui/Cargo.lock'
sleep "${sec}"
echo -e '         35.15K  69%   33.52MB/s    0:00:00 (xfr#2, to-chk=19/22)'
sleep "${sec}"
echo -e 'gui/Cargo.toml'
sleep "${sec}"
echo -e '         35.34K  70%   33.70MB/s    0:00:00 (xfr#3, to-chk=18/22)'
sleep "${sec}"
echo -e 'gui/aaa'
sleep "${sec}"
echo -e '         40.26K  80%   38.39MB/s    0:00:00 (xfr#4, to-chk=17/22)'
sleep "${sec}"
echo -e 'gui/bbb'
sleep "${sec}"
echo -e '         43.98K  87%   41.93MB/s    0:00:00 (xfr#5, to-chk=16/22)'
sleep "${sec}"
echo -e 'gui/.git/'
sleep "${sec}"
echo -e 'gui/.git/HEAD'
sleep "${sec}"
echo -e '         44.00K  87%   41.96MB/s    0:00:00 (xfr#6, to-chk=13/22)'
sleep "${sec}"
echo -e 'gui/.git/config'
sleep "${sec}"
echo -e '         44.14K  87%   42.09MB/s    0:00:00 (xfr#7, to-chk=12/22)'
sleep "${sec}"
echo -e 'gui/.git/description'
sleep "${sec}"
echo -e '         44.21K  87%   42.16MB/s    0:00:00 (xfr#8, to-chk=11/22)'
sleep "${sec}"
echo -e 'gui/.git/hooks/'
sleep "${sec}"
echo -e 'gui/.git/hooks/README.sample'
sleep "${sec}"
echo -e '         44.39K  88%   42.33MB/s    0:00:00 (xfr#9, to-chk=6/22) '
sleep "${sec}"
echo -e 'gui/.git/info/'
sleep "${sec}"
echo -e 'gui/.git/info/exclude'
sleep "${sec}"
echo -e '         44.50K  88%   42.43MB/s    0:00:00 (xfr#10, to-chk=5/22)'
sleep "${sec}"
echo -e 'gui/.git/objects/'
sleep "${sec}"
echo -e 'gui/.git/objects/info/'
sleep "${sec}"
echo -e 'gui/.git/objects/pack/'
sleep "${sec}"
echo -e 'gui/.git/refs/'
sleep "${sec}"
echo -e 'gui/.git/refs/heads/'
sleep "${sec}"
echo -e 'gui/.git/refs/tags/'
sleep "${sec}"
echo -e 'gui/src/'
sleep "${sec}"
echo -e 'gui/src/main.rs'
sleep "${sec}"
echo -e '         50.30K 100%   47.96MB/s    0:00:00 (xfr#11, to-chk=0/22)         50.30K 100%   11.99MB/s    0:00:00 (xfr#11, to-chk=0/22)         50.30K 100%   11.99MB/s    0:00:00 (xfr#11, to-chk=0/22)         50.30K 100%   11.99MB/s    0:00:00 (xfr#11, to-chk=0/22)         50.30K 100%   11.99MB/s    0:00:00 (xfr#11, to-chk=0/22)         50.30K 100%   11.99MB/s    0:00:00 (xfr#11, to-chk=0/22)         50.30K 100%   11.99MB/s    0:00:00 (xfr#11, to-chk=0/22)         50.30K 100%   11.99MB/s    0:00:00 (xfr#11, to-chk=0/22)         50.30K 100%   11.99MB/s    0:00:00 (xfr#11, to-chk=0/22)         50.30K 100%   11.99MB/s    0:00:00 (xfr#11, to-chk=0/22)         50.30K 100%   11.99MB/s    0:00:00 (xfr#11, to-chk=0/22)         50.30K 100%   11.99MB/s    0:00:00 (xfr#11, to-chk=0/22)'

