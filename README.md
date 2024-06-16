## nights and weekends starts today and ends in 6 weeks.

![image](https://github.com/malted/buildspace-timer/assets/59726149/daf4ab0b-0838-4dc8-b670-dfca2866282c)

includes quicklink to [sage](https://sage.buildspace.so):

![image](https://github.com/malted/buildspace-timer/assets/59726149/0656027d-0177-4066-9f61-57ed6e5e229b)


### install:
```bash
cargo install --git https://github.com/malted/buildspace-timer

sudo tee /Library/LaunchDaemons/dev.malted.buildspace-timer.plist <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>dev.malted.buildspace-timer</string>

    <key>ProgramArguments</key>
    <array>
        <string>$HOME/.cargo/bin/buildspace-timer</string>
    </array>

    <key>RunAtLoad</key>
    <true/>
    
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
EOF

sudo chmod 644 /Library/LaunchDaemons/dev.malted.buildspace-timer.plist

sudo launchctl load /Library/LaunchDaemons/dev.malted.buildspace-timer.plist
```
