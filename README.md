# VHS Server Patcher
![image](https://github.com/LuisMayo/vhs-patcher-redirector/assets/20229636/134f20ff-d87d-46d4-98d4-e2d668e982a3)
An app to patch your Video Horror Society Steam installation to use private/communiy servers, including the END server. More info here: https://github.com/LuisMayo/vhs-alternative-server/
This is intendend for Linux users (or for Windows Users who are developers of the server). If you're a windows user please check the Hosts redirecton app: https://github.com/SkelXton/VHS-Redirector-Scripts/

## Usage
Download latest version from releases and run it. Check compiling from source section if there isn't a build for your OS.

## Build from source
### Requirements.
- Node.JS + npm
- Rust + Cargo (Check Rustup)
- Tauri requirements: https://tauri.app/v1/guides/getting-started/prerequisites/

### Compiling
1. Clone the repo
   `git clone https://github.com/LuisMayo/vhs-server-patcher`
2. Install dependencies
  `cd vhs-server-patcher; npm i`
3. If you want to just build it...
  `npx tauri build`
   Or if you prefer to run it for development purposes
   `npx tauri dev`

## Contributing
Since this is a tiny project we don't have strict rules about contributions. Just open a Pull Request to fix any of the project issues or any improvement you have percieved on your own. Any contributions which improve or fix the project will be accepted as long as they don't deviate too much from the project objectives. If you have doubts about whether the PR would be accepted or not you can open an issue before coding to ask for my opinion.
