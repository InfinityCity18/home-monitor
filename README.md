# Home monitor

![Preview of website](/preview.png)

Application that monitors and plots temperature, humidity, motion and light in your home, with choosable periods: 24h, week or month.
Designed for usage with: https://github.com/InfinityCity18/temp-humidity-monitor

To run the container use:
`sudo docker run -p 8137:8137 -p 8138:8138 -v ./data.db:/home-monitor/website/data.db -v ./consts.rs:/home-monitor/website/src/consts.rs infinitycity/home-monitor:latest`

```
git clone https://github.com/InfinityCity18/home-monitor.git (or just download Dockerfile)
docker build -t home-monitor .
sudo docker run -p 8137:8137 -p 8138:8138 -v ./data.db:/home-monitor/website/data.db -v ./consts.rs:/home-monitor/website/src/consts.rs home-monitor
```
