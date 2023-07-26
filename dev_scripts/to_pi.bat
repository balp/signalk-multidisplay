cross build --release --target arm-unknown-linux-gnueabihf
scp target\arm-unknown-linux-gnueabihf\release\signalk-multidisplay pi@192.168.1.102:/var/tmp/
ssh pi@192.168.1.102 chmod +x /var/tmp/signalk-multidisplay
