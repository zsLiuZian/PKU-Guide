# PKU-Guide
- 顾名思义，PKU-Guide是一款北大导览小助手，主要提供路线规划引导功能，具体而言可以提供两种服务。其一是计算两点之间的最短路线，其二是规划一条游览校园的路线，并且均在地图上以渐变的方式呈现，其中地点支持文本输入或地图选点。此外，整个过程（包括何时退出）完全使用自然语言进行交互控制，因此还可将其视为普通的聊天机器人进行任何交流。
- 基本形式如下图，详情可见演示视频。
- 视频链接：https://disk.pku.edu.cn/link/AAF63C6B4768A0451EB34F62ABB1B93B38
![image](https://github.com/user-attachments/assets/9926b260-476c-4d85-8422-3ea4e1c2dc08)

- 项目运行需要将./src/agent.rs的第10行和第140行的<API-Key>分别替换为高德地图和DeepSeek的API-Key
