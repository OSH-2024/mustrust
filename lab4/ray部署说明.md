## Ray简介
Ray 是伯克利大学 RISELab 研发的一个简单高效的分布式计算引擎，为开发者提供了简单通用的API来构建分布式程序。

Ray 能够让开发者轻松地构建分布式程序，靠的是通过简单的API来将计算任务分解为以下的计算原语来执行：（以下两段来自实验文档）

Task：一个无状态的计算任务（函数表示）。Ray 允许异步执行任意函数。这些"remote function"（Task）的开销非常低，可以在毫秒内执行，并且可以自动向集群添加节点并调度任务，非常适合扩展计算密集型应用程序和服务。

Actor：一个有状态的计算任务（类表示）。Actor 模型是一个强大的异步编程范例（支持微服务），可以在本地和远程无缝工作。Actor 本质上是一个有状态的 Worker（或 service）。当一个新的 Actor 被实例化时，就创建一个新的 Worker，并将该 Actor 的方法调度到这个特定的 Worker，也可以对 Worker 的状态进行访问和修改。
要获取更详细的关于Ray的基础结构的信息，可以参考原论文：[[1712.05889] Ray: A Distributed Framework for Emerging AI Applications (arxiv.org)](https://arxiv.org/abs/1712.05889)

## Docker简介

### 什么是Docker？
Docker是一种开源的容器化平台，用于开发、运输和运行应用程序。它通过将应用程序及其依赖项打包到一个容器中，确保应用程序在任何环境中都能一致地运行。容器类似于轻量级的虚拟机，但它们更高效，因为它们共享主机操作系统的内核。

### Docker的基本概念
- **镜像（Image）**：一个只读的模板，其中包含创建容器所需的所有内容（例如代码、运行时、库等）。
- **容器（Container）**：镜像的一个运行实例。容器是独立的、可执行的软件包，包含应用程序及其依赖项。
- **Docker Hub**：一个公共的注册表，用于存储和分发Docker镜像。你可以从Docker Hub中拉取现有的镜像，或者将自己的镜像推送到Docker Hub。

### Docker Desktop for Windows
为了在Windows系统上使用Docker，我们可以安装Docker Desktop。Docker Desktop是一个易于使用的应用程序，允许你在Windows上构建、运行和共享容器。

### Docker Desktop的工作原理
Docker Desktop在Windows上运行时，主要依赖于Windows Subsystem for Linux 2 (WSL 2)。WSL 2是微软提供的一个轻量级的Linux内核，可以在Windows上运行Linux应用程序。Docker Desktop利用WSL 2来创建一个Linux环境，从而运行Linux容器。

通过WSL 2，Docker Desktop能够在Windows上实现与Linux系统相同的性能和兼容性。这意味着你可以在Windows上开发、测试和运行与Linux环境相同的容器，而无需担心跨平台兼容性问题。

### 总结
Docker通过容器化技术，使得应用程序可以在各种环境中一致地运行。对于Windows用户，Docker Desktop提供了一个方便的工具来管理和运行容器。基于WSL 2的技术，Docker Desktop在Windows上实现了与Linux系统相同的性能和兼容性，这让开发者能够轻松地在Windows上进行容器化开发和部署。

## Windows下基于Docker的Ray部署安装说明


### 安装环境

windows11 + WSL2

如果你的windows系统中还没有安装WSL，可以查看这篇[超详细Windows10/Windows11 子系统（WSL2）安装Ubuntu20.04（带桌面环境）](https://blog.csdn.net/weixin_44301630/article/details/122390018)，装完WSL即可，无需桌面环境。

### 安装Docker Desktop

1. **下载Docker Desktop**：访问[Docker官方网站](https://www.docker.com/products/docker-desktop)，下载适用于Windows的Docker Desktop安装包。（官网下载比较慢，睿客网盘下载链接：[Docker Desktop Installer下载链接](https://rec.ustc.edu.cn/share/c7148650-3236-11ef-b687-a55c4de42197)）
2. **安装Docker Desktop**：
   - 双击下载的安装包，按照提示进行安装。
   - 安装过程中会提示你启用Windows Subsystem for Linux 2 (WSL 2)，这也是Docker在Windows上运行的推荐方式。
3. **启动Docker Desktop**：安装完成后，启动Docker Desktop应用程序。初次启动时，可能需要花费几分钟来初始化环境。
4. **验证安装**：打开命令提示符或PowerShell，输入以下命令来验证Docker是否安装成功：
   ```shell
   docker --version
   ```
   你应该会看到Docker的版本信息，这表示Docker已经成功安装并运行。
5. **Docker换源**：在Docker Desktop图形界面下，换源比较简单。只要在`Setting`的`Docker Engine`选项下直接编辑json文件添加下面一行即可，注意上一行末尾加`,`这里使用科大源。
    ```json
    "registry-mirrors": [
        "https://docker.mirrors.ustc.edu.cn/"
    ]
    ```
![alt text](src/25c018bf99946fe7f32645df4a19d183.png)

### 部署Ray
Ray的下载安装有多种方式，这里主要介绍通过**拉取Docker镜像**的方式，并列出其他几种常见方法，更多详细内容可以参考官网: [Ray安装指南](https://docs.ray.io/en/latest/ray-overview/installation.html#building-ray-from-source)。

#### 从Docker Hub中拉取镜像
我们将从Docker Hub中拉取`rayproject/ray`镜像，该镜像已经打包部署了Ray及其运行环境，包括Linux系统、Python、Anaconda及所需的Python库等。（Ray Docker镜像地址为：[Docker Hub Ray镜像](https://hub.docker.com/r/rayproject/ray)）

**注意**：目前测试未发现测试过程中Python版本对结果有显著影响，但在issue中发现Python 3.6存在一些bug，建议使用3.7及以上版本。

#### 安装步骤：

1. 安装Docker Desktop，确保Docker正常运行。
2. 拉取Ray镜像，命令为：`docker pull rayproject/ray`（可用`docker images`或在Docker Desktop的`Images`选项中，查看当前所有镜像，以确认Ray镜像是否成功引入）。
![alt text](src/image.png)
![alt text](src/c4a0b46b99d024dad6e8407546b0da41.png)

#### 基于镜像创建并运行容器：

在powershell中输入以下命令：
```shell
docker run --shm-size=4G -t -i -p 8265:8265 -p 3000:3000 -p 9000:9000 -p 6379:6379 rayproject/ray
```

**参数说明：**
- `--shm-size`: 推荐使用4G及以上（配置不足时可以适当减少），此参数可自定义。省略此参数则使用默认空间划分。
- `-i`: 交互式操作。
- `-t`: 终端。
- `-p`: 端口映射，格式为主机端口:容器端口，可多次使用。8265端口为dashboard默认端口，3000端口为Grafana默认端口，9000端口为Prometheus默认端口，6379端口为Ray头结点连接（用于分布式部署）默认端口。

此时，命令提示符从Windows盘符地址变成了容器的Shell提示符：
```shell
(base) ray@f01a2a6d0145:~$
```
这表明你已经进入了Docker容器的交互式终端环境，提示符的一部分内容会显示当前用户和主机名（容器ID的一部分），解释如下：
- **(base)**： 这表明当前激活的是一个Conda环境，通常是base环境。
- **ray**：这是你在容器内的用户名。
- **f01a2a6d0145**： 这是容器的部分ID，用于标识容器。
- **~$**： 这是容器内当前的工作目录，~代表用户的主目录。

现在，你已经基于ray镜像成功地创建并运行了一个Docker容器，并已经针对我们未来要用到的数据可视化工具Prometheus与Grafana分配了端口。

#### 后续操作：
- 若要重新打开容器，用`docker start`命令，参数为容器ID或容器名：`docker start [OPTIONS] CONTAINER [CONTAINER...]`。
- 可使用`docker commit`将修改后的容器提交为镜像的新版本，指令格式为：`docker commit [OPTIONS] CONTAINER [REPOSITORY[:TAG]]`，其中OPTIONS为可选项，CONTAINER为容器ID或容器名，REPOSITORY为新镜像的名字，TAG为新镜像的标签，若不指定则默认为latest。
- 使用`docker cp`命令在本地与Docker容器间拷贝文件：
  - 本地文件拷贝到容器：`docker cp <本地文件路径> <容器名或ID>:<docker目标路径>`
  - 容器文件拷贝到本地：`docker cp <容器名或ID>:<docker源路径> <本地文件路径>`
- 可选：在VSCode中下载Docker插件，该插件提供部分图形化功能，特别是访问容器文件列表与编辑容器文件的功能，非常便捷。

当然，这些功能在Docker Desktop里面都有对应的图形化实现，可以自行探索。

#### 附1：直接安装Ray包
通过Pypi下载，可以直接将Ray作为一个Python包来安装（`ray[default]`为默认部分，可选择`ray[air]`加入Ray的AI支持项）：
```shell
# 安装Ray并支持dashboard和集群启动
pip install -U "ray[default]"
# 安装Ray及其AI运行时的依赖
pip install -U "ray[air]"
```

#### 附2：从源码安装Ray
拉取Ray的GitHub仓库源码：
```shell
git clone git@github.com:ray-project/ray.git
```
根据官网指南进行测试：
```shell
python -m pytest -v python/ray/tests/test_mini.py
```
**测试程序运行若报错：**
- 如果报`pytest`不存在，需要安装：`pip install pytest`。
- 如果报`ERROR: file or directory not found: python/ray/tests/test_mini.py`，需要在Git仓库根目录下运行。
- 如果报`ImportError: cannot import name 'find_available_port' from 'ray._private.test_utils'`，需要进入`python/ray/tests/conftest.py:28`，并将`find_available_port`注释掉，实测可以正常通过`PASS`。

### 下载Prometheus与Grafana等配件

在上一节中，你已经基于ray镜像成功地创建并运行了一个Docker容器，并已经针对我们未来要用到的数据可视化工具Prometheus与Grafana分配了端口。现在，让我们一起来下载这两个工具。

#### Q：为什么要采用数据可视化工具Prometheus与Grafana？
尽管Docker Dashboard已经提供了基本的可视化管理功能，但添加Prometheus和Grafana能够提供更强大和详细的数据可视化与监控功能，尤其在性能测试和优化过程中非常有用。以下是原因简要说明：

1. **详细数据监控**：
   - **Prometheus**：专注于实时数据收集和存储，特别适合监控系统和服务的性能指标。
   - **Grafana**：提供强大的数据可视化功能，可以将Prometheus收集的数据以图表形式展示，便于分析和评估。

2. **性能指标跟踪**：
   - Prometheus和Grafana可以帮助追踪和展示多个性能指标，如延迟、IOPS等，便于进行细致的性能测试和分析。

3. **历史数据和趋势分析**：
   - 这些工具能存储历史数据，方便进行趋势分析和长期性能评估。

4. **灵活性和可扩展性**：
   - 允许定制和扩展监控项，满足实验需求的多样性和复杂性。

通过这些工具，可以全面监控和优化Ray的性能，确保实验要求得到充分满足，且能清晰地展示优化效果。

#### 下载步骤

在容器的shell提示符中继续输入命令

下载Prometheus的可执行文件，然后解压：
```shell
wget https://github.com/prometheus/prometheus/releases/download/v2.37.8/prometheus-2.37.8.linux-amd64.tar.gz
tar -xzvf prometheus-*.tar.gz
```
下载Grafana的可执行文件，然后解压：
```shell
wget https://dl.grafana.com/enterprise/release/grafana-enterprise-9.5.2.linux-amd64.tar.gz
tar -xzvf grafana-enterprise-9.5.2.linux-amd64.tar.gz
```

### 单机运行Ray

在之前的步骤中，我们已经完成了Ray的安装和部署，现在我们将启动Ray。

#### 启动Ray
首先，在命令行中输入以下命令来启动Ray：

```shell
ray start --head --port=6379 --dashboard-host=0.0.0.0
```

**注意**：必须将`dashboard-host`设为`0.0.0.0`，这样才能在Docker外部的主机中访问dashboard。

此时，Ray服务已经启动。

#### 启动Prometheus和Grafana
接下来，我们需要启动Prometheus和Grafana来进行数据监控和可视化。

新开一个命令行终端（可以使用`docker exec`命令，也可以在Docker Desktop中按照下图操作）
![alt text](src/image1.png)
![alt text](src/image2.png)

然后从当前目录，进入安装好的Prometheus目录（例如`prometheus-*.linux-amd64`），输入以下命令：

```shell
./prometheus --config.file=/tmp/ray/session_latest/metrics/prometheus/prometheus.yml
```

同样地，在安装好的Grafana目录下（例如`grafana-9.5.2`，版本号可能因情况而异），新开一个命令行终端（可以使用`docker exec`命令），然后输入以下命令：

```shell
./bin/grafana-server --config /tmp/ray/session_latest/metrics/grafana/grafana.ini web
```

此时，Prometheus和Grafana的服务也已经启动。我们可以在浏览器中输入`127.0.0.1:8265`来查看Ray的dashboard。

![alt text](src/image3.png)

#### 运行测试程序

新建一个测试程序test.py，测试Ray是否正常运行

```python
import ray

ray.init(dashboard_host="0.0.0.0")

# Define the square task.
@ray.remote
def square(x):
    return x * x

# Launch four parallel square tasks.
futures = [square.remote(i) for i in range(4)]

# Retrieve results.
print(ray.get(futures))
# -> [0, 1, 4, 9]
```
此时，如果你还没厘清Windows, WSL, Linux, Docker, Docker容器的区别，请你仔细阅读下面一段话。

- Docker基于Linux系统运行，在我们的实验中，它使用的是Windows自带的WSL2（Windows Subsystem for Linux 2）。当我们在Windows上安装Docker Desktop时，实际上是为WSL2环境安装了Docker。Docker利用WSL2提供的Linux环境来创建Docker容器，而Docker Desktop则充当了Docker容器与Windows系统之间的桥梁。

- 在本文中，我们拉取了一个Ray的Docker镜像，实际上是将一个预先配置好了Python和Ray的Linux子系统作为Docker容器启动。这个容器可以看作是一个轻量级的虚拟环境，其中已经包含了所需的Python环境和Ray框架，使得用户可以快速启动和运行Ray应用程序，而无需担心配置和依赖项的安装。

如何将test.py放到容器中启动呢？Docker Desktop提供了非常简便的图形化方式，只要在容器的`file`选项中右击相应的文件夹，即可将你写在Windows系统上的文件方便地直接传入Docker容器中，跨过了WSL这一层。

![alt text](src/8e68b2672d118295c3e7de967ff3e4cb.png)

上传完毕后，在Docker容器中，进入文件所在目录，像在本地运行python代码一样输入
``` shell
python3 test.py
```
即可成功运行python程序。我们可以在命令行输出和dashboard中分别查看程序的运行情况，若运行正常，说明我们单机部署已经成功。

![alt text](src/image4.png)

可在命令行中用`ray stop`命令停止Ray服务

并不是。Ray的分布式部署方式并不依赖于在同一台PC上创建多个Docker容器来模拟多台机器，而是通过在实际的多台机器上运行多个Ray节点来实现分布式部署。以下是简要的分布式部署过程：

### 分布式部署Ray

#### 1. 准备多台机器
确保你有多台物理或虚拟的计算机，这些计算机可以在同一个局域网内通信。

#### 2. 在每台机器上安装Docker和Ray
在每台机器上按照之前的步骤安装Docker，并拉取Ray的Docker镜像。

```shell
# 在每台机器上运行
docker pull rayproject/ray
```

#### 3. 启动主节点（Head Node）
在其中一台机器上启动Ray的主节点：

```shell
ray start --head --port=6379 --dashboard-host=0.0.0.0
```

#### 4. 启动从节点（Worker Nodes）
在其他机器上连接到主节点，启动Ray从节点：

```shell
ray start --address='主节点IP:6379'
```
其中，`主节点IP`是你在第3步中启动主节点的机器的IP地址。

#### 5. 验证分布式部署
使用`ray status`命令来检查集群状态，确保所有节点都已正确连接。

```shell
ray status
```

#### 6. 启动监控工具
在主节点上启动Prometheus和Grafana，以便监控整个集群的性能。

```shell
# Prometheus
./prometheus --config.file=/tmp/ray/session_latest/metrics/prometheus/prometheus.yml

# Grafana
./bin/grafana-server --config /tmp/ray/session_latest/metrics/grafana/grafana.ini web
```

#### 总结
Ray的分布式部署是在实际的多台机器上运行多个Ray节点，通过网络连接实现分布式计算。每台机器上运行的Ray节点可以协同工作，形成一个统一的分布式计算集群。这样可以充分利用多台机器的计算资源，实现高效的分布式计算。