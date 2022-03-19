# namehax
## What is namehax?
Namehax is a tool that lets you bypass the flagged name screen on the league client. [example](https://i.imgur.com/QyqAWEn.png). It does so by patching the string 'nameChangeFlag' in memory, causing the internal json deserializer to default the value to false. It has a hard-coded memory offset but you can use `namehax.exe -d "nameChangeFlag"` to dump a new offset, or any other string you wish to patch. You can then use `namehax.exe -o <offset>` to use that custom offset.

## How to use?
Just run `namehax.exe` after opening the league cient. Then login.