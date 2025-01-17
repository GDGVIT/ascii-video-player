<p align="center">
<a href="https://dscvit.com">
	<img width="400" src="https://user-images.githubusercontent.com/56252312/159312411-58410727-3933-4224-b43e-4e9b627838a3.png#gh-light-mode-only" alt="GDSC VIT"/>
</a>
	<h2 align="center"> AsciiFlix </h2>
	<p align="center">
	<img width="400" src="https://github.com/user-attachments/assets/dade11d9-5498-46be-84e4-bf74c5a22da9" alt="AsciiFlix" />
	<h4 align="center"> Play videos in the terminal <h4>
</p>

---
[![Join Us](https://img.shields.io/badge/Join%20Us-Developer%20Student%20Clubs-red)](https://dsc.community.dev/vellore-institute-of-technology/)
[![Discord Chat](https://img.shields.io/discord/760928671698649098.svg)](https://discord.gg/498KVdSKWR)

[![DOCS](https://img.shields.io/badge/Documentation-see%20docs-green?style=flat-square&logo=appveyor)](INSERT_LINK_FOR_DOCS_HERE) 
  [![UI ](https://img.shields.io/badge/User%20Interface-Link%20to%20UI-orange?style=flat-square&logo=appveyor)](INSERT_UI_LINK_HERE)

## Description
- This video player uses `opencv` to convert frames of a video to text in real time.
- There are two threads, one to encode each frame into text, and another to display each frame at the right fps.
- It uses a `std::mpsc::sync_channel` to communicate between the threads.

## Running

First, build the program with 
```bash
cargo build --release
```

then run the program with

```bash
cargo run <video file>
```

## Controls

| Key               | Action                   |
|-------------------|--------------------------|
| `Q`               | Quit                     |
| `Left_arrow_key`  | Slow down by 2x          |
| `Right_arrow_key` | Speed up by 2x           |
| `Space`           | Pause                    |


## Demo

https://github.com/user-attachments/assets/d5f15159-97d2-4f0c-833c-df54a4c9e9e9

## Contributors
<table>
	<tr align="center">
		<td>
		Kurian Jojo
		<p align="center">
			<img src = "https://github.com/user-attachments/assets/2c500751-9c0a-4c50-95bf-292c39a56b3e" width="150" height="150" alt="Kurian Jojo">
		</p>
			<p align="center">
				<a href = "https://github.com/polyesterswing">
					<img src = "http://www.iconninja.com/files/241/825/211/round-collaboration-social-github-code-circle-network-icon.svg" width="36" height = "36" alt="GitHub"/>
				</a>
				<a href = "https://www.linkedin.com/in/kurian-jojo-544a54215/">
					<img src = "http://www.iconninja.com/files/863/607/751/network-linkedin-social-connection-circular-circle-media-icon.svg" width="36" height="36" alt="LinkedIn"/>
				</a>
			</p>
		</td>
	</tr>
</table>


<p align="center">
	Made with ❤ by <a href="https://dscvit.com">GDSC-VIT</a>
</p>
