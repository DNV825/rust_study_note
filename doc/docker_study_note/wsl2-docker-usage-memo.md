# WSL2-docker 使い方メモ

```shell
wsluser@PC:~$ curl -fsSL https://get.docker.com -o get-docker.sh
wsluser@PC:~$ sudo sh get-docker.sh
[sudo] password for wsluser:
# Executing docker install script, commit: 4c94a56999e10efcf48c5b8e3f6afea464f9108e
 
WSL DETECTED: We recommend using Docker Desktop for Windows.
Please get Docker Desktop from https://www.docker.com/products/docker-desktop/
 
You may press Ctrl+C now to abort this script.
+ sleep 20
+ sh -c apt-get -qq update >/dev/null
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/focal/InRelease  Temporary failure resolving 'archive.ubuntu.com'
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/focal-updates/InRelease  Temporary failure resolving 'archive.ubuntu.com'
W: Failed to fetch http://archive.ubuntu.com/ubuntu/dists/focal-backports/InRelease  Temporary failure resolving 'archive.ubuntu.com'
W: Failed to fetch http://security.ubuntu.com/ubuntu/dists/focal-security/InRelease  Temporary failure resolving 'security.ubuntu.com'
W: Some index files failed to download. They have been ignored, or old ones used instead.
+ sh -c DEBIAN_FRONTEND=noninteractive apt-get -y -qq install ca-certificates curl >/dev/null
+ sh -c install -m 0755 -d /etc/apt/keyrings
+ sh -c curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
curl: (6) Could not resolve host: download.docker.com
```

## docker を入れてみよう

```shell
wsluser@PC:~$ curl -fsSL https://get.docker.com -o get-docker.sh
wsluser@PC:~$ sudo sh get-docker.sh
[sudo] password for wsluser:
# Executing docker install script, commit: 4c94a56999e10efcf48c5b8e3f6afea464f9108e
 
WSL DETECTED: We recommend using Docker Desktop for Windows.
Please get Docker Desktop from https://www.docker.com/products/docker-desktop/
 
You may press Ctrl+C now to abort this script.
+ sleep 20
+ sh -c apt-get -qq update >/dev/null
+ sh -c DEBIAN_FRONTEND=noninteractive apt-get -y -qq install ca-certificates curl >/dev/null
+ sh -c install -m 0755 -d /etc/apt/keyrings
+ sh -c curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
+ sh -c chmod a+r /etc/apt/keyrings/docker.asc
+ sh -c echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu focal stable" > /etc/apt/sources.list.d/docker.list
+ sh -c apt-get -qq update >/dev/null
+ sh -c DEBIAN_FRONTEND=noninteractive apt-get -y -qq install docker-ce docker-ce-cli containerd.io docker-compose-plugin docker-ce-rootless-extras docker-buildx-plugin >/dev/null
+ sh -c docker version
Client: Docker Engine - Community
 Version:           28.0.0
 API version:       1.48
 Go version:        go1.23.6
 Git commit:        f9ced58
 Built:             Wed Feb 19 22:10:39 2025
 OS/Arch:           linux/amd64
 Context:           default
 
Server: Docker Engine - Community
 Engine:
  Version:          28.0.0
  API version:      1.48 (minimum version 1.24)
  Go version:       go1.23.6
  Git commit:       af898ab
  Built:            Wed Feb 19 22:10:39 2025
  OS/Arch:          linux/amd64
  Experimental:     false
 containerd:
  Version:          1.7.25
  GitCommit:        bcc810d6b9066471b0b6fa75f557a15a1cbf31bb
 runc:
  Version:          1.2.4
  GitCommit:        v1.2.4-0-g6c52b3f
 docker-init:
  Version:          0.19.0
  GitCommit:        de40ad0
 
================================================================================
 
To run Docker as a non-privileged user, consider setting up the
Docker daemon in rootless mode for your user:
 
    dockerd-rootless-setuptool.sh install
 
Visit https://docs.docker.com/go/rootless/ to learn about rootless mode.
 
To run the Docker daemon as a fully privileged service, but granting non-root
users access, refer to https://docs.docker.com/go/daemon-access/
 
WARNING: Access to the remote API on a privileged Docker daemon is equivalent
         to root access on the host. Refer to the 'Docker daemon attack surface'
         documentation for details: https://docs.docker.com/go/attack-surface/
 
================================================================================
 
# docker がインストールされているかの確認。バージョンが表示されれば OK。
wsluser@PC:~$ docker -v
Docker version 28.0.0, build f9ced58
 
# そのままだと sudo をつけなければ docker を動かせないため、docker グループに ユーザーを追加し、 sudo なしで動かせるようにする。
wsluser@PC:~$ getent group docker
docker:x:998:
wsluser@PC:~$ sudo usermod -aG docker $USER
wsluser@PC:~$ getent group docker
docker:x:998:wsluser
```

```shell
PS C:\Users\xxxx> wsl --shutdown
```

```shell
wsluser@PC:~$ service docker status
● docker.service - Docker Application Container Engine
     Loaded: loaded (/lib/systemd/system/docker.service; enabled; vendor preset: enabled)
     Active: active (running) since Fri 2025-02-21 16:15:38 JST; 9s ago
TriggeredBy: ● docker.socket
       Docs: https://docs.docker.com
   Main PID: 454 (dockerd)
      Tasks: 18
     Memory: 99.0M
     CGroup: /system.slice/docker.service
             └─454 /usr/bin/dockerd -H fd:// --containerd=/run/containerd/containerd.sock
 
Feb 21 16:15:38 PC dockerd[454]: time="2025-02-21T16:15:38.952278164+09:00" level=warning msg="WARNING: No>
Feb 21 16:15:38 PC dockerd[454]: time="2025-02-21T16:15:38.952307671+09:00" level=warning msg="WARNING: No>
Feb 21 16:15:38 PC dockerd[454]: time="2025-02-21T16:15:38.952312968+09:00" level=warning msg="WARNING: No>
Feb 21 16:15:38 PC dockerd[454]: time="2025-02-21T16:15:38.952315827+09:00" level=warning msg="WARNING: No>
Feb 21 16:15:38 PC dockerd[454]: time="2025-02-21T16:15:38.952328223+09:00" level=info msg="Docker daemon">
Feb 21 16:15:38 PC dockerd[454]: time="2025-02-21T16:15:38.952583437+09:00" level=info msg="Initializing b>
Feb 21 16:15:38 PC dockerd[454]: time="2025-02-21T16:15:38.995653794+09:00" level=info msg="Completed buil>
Feb 21 16:15:38 PC dockerd[454]: time="2025-02-21T16:15:38.999012661+09:00" level=info msg="Daemon has com>
Feb 21 16:15:38 PC dockerd[454]: time="2025-02-21T16:15:38.999155339+09:00" level=info msg="API listen on >
Feb 21 16:15:38 PC systemd[1]: Started Docker Application Container Engine.
```

```shell
wsluser@PC:~$ docker container run hello-world
Unable to find image 'hello-world:latest' locally
latest: Pulling from library/hello-world
e6590344b1a5: Pull complete
Digest: sha256:e0b569a5163a5e6be84e210a2587e7d447e08f87a0e90798363fa44a0464a1e8
Status: Downloaded newer image for hello-world:latest
 
Hello from Docker!
This message shows that your installation appears to be working correctly.
 
To generate this message, Docker took the following steps:
 1. The Docker client contacted the Docker daemon.
 2. The Docker daemon pulled the "hello-world" image from the Docker Hub.
    (amd64)
 3. The Docker daemon created a new container from that image which runs the
    executable that produces the output you are currently reading.
 4. The Docker daemon streamed that output to the Docker client, which sent it
    to your terminal.
 
To try something more ambitious, you can run an Ubuntu container with:
wsluser@PC:~$ docker run -it ubuntu bash
 
Share images, automate workflows, and more with a free Docker ID:
 https://hub.docker.com/
 
For more examples and ideas, visit:
 https://docs.docker.com/get-started/
```

```shell
wsluser@PC:~$ docker images
REPOSITORY    TAG       IMAGE ID       CREATED       SIZE
ubuntu        latest    a04dc4851cbc   4 weeks ago   78.1MB
hello-world   latest    74cc54e27dc4   5 weeks ago   10.1kB
 
wsluser@PC:~$ docker ps -a
CONTAINER ID   IMAGE         COMMAND    CREATED      STATUS                  PORTS     NAMES
fb353515d033   hello-world   "/hello"   5 days ago   Exited (0) 5 days ago             compassionate_tu
 
# コンテナが存在するとイメージを削除できない。
wsluser@PC:~$ docker image rm 74cc54e27dc4
Error response from daemon: conflict: unable to delete 74cc54e27dc4 (must be forced) - image is being used by stopped container fb353515d033
 
# 先にコンテナを削除し、
wsluser@PC:~$ docker container rm fb353515d033
fb353515d033
 
wsluser@PC:~$ docker ps -a
CONTAINER ID   IMAGE     COMMAND   CREATED   STATUS    PORTS     NAMES
 
# その後イメージを削除する。
wsluser@PC:~$ docker image rm 74cc54e27dc4
Untagged: hello-world:latest
Untagged: hello-world@sha256:e0b569a5163a5e6be84e210a2587e7d447e08f87a0e90798363fa44a0464a1e8
Deleted: sha256:74cc54e27dc41bb10dc4b2226072d469509f2f22f1a3ce74f4a59661a1d44602
Deleted: sha256:63a41026379f4391a306242eb0b9f26dc3550d863b7fdbb97d899f6eb89efe72
 
wsluser@PC:~$ docker images
REPOSITORY   TAG       IMAGE ID       CREATED       SIZE
ubuntu       latest    a04dc4851cbc   4 weeks ago   78.1MB
```

イメージからコンテナを作り、コンテナを動かす（Up）。run はイメージを指定する。

```shell
wsluser@PC:~/prj/docker$ docker container run hello-world
Unable to find image 'hello-world:latest' locally
latest: Pulling from library/hello-world
e6590344b1a5: Pull complete
Digest: sha256:e0b569a5163a5e6be84e210a2587e7d447e08f87a0e90798363fa44a0464a1e8
Status: Downloaded newer image for hello-world:latest
 
Hello from Docker!
This message shows that your installation appears to be working correctly.
 
To generate this message, Docker took the following steps:
 1. The Docker client contacted the Docker daemon.
 2. The Docker daemon pulled the "hello-world" image from the Docker Hub.
    (amd64)
 3. The Docker daemon created a new container from that image which runs the
    executable that produces the output you are currently reading.
 4. The Docker daemon streamed that output to the Docker client, which sent it
    to your terminal.
 
To try something more ambitious, you can run an Ubuntu container with:
 $ docker run -it ubuntu bash
 
Share images, automate workflows, and more with a free Docker ID:
 https://hub.docker.com/
 
For more examples and ideas, visit:
 https://docs.docker.com/get-started/
```

Cmd にデフォルトコマンドが定義されている。

```shell
wsluser@PC:~/prj/docker$ docker image inspect ubuntu:20.04
[
    {
        "Id": "sha256:6013ae1a63c2ee58a8949f03c6366a3ef6a2f386a7db27d86de2de965e9f450b",
        "RepoTags": [
            "ubuntu:20.04"
        ],
        "RepoDigests": [
            "ubuntu@sha256:8e5c4f0285ecbb4ead070431d29b576a530d3166df73ec44affc1cd27555141b"
        ],
        "Parent": "",
        "Comment": "",
        "Created": "2024-10-11T03:38:27.357079367Z",
        "DockerVersion": "24.0.7",
        "Author": "",
        "Config": {
            "Hostname": "",
            "Domainname": "",
            "User": "",
            "AttachStdin": false,
            "AttachStdout": false,
            "AttachStderr": false,
            "Tty": false,
            "OpenStdin": false,
            "StdinOnce": false,
            "Env": [
                "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
            ],
            "Cmd": [
                "/bin/bash"
            ],
            "Image": "sha256:4067b49cecb392ddf6769415769fafd51c92600e6290df594da41917d3d404c2",
            "Volumes": null,
            "WorkingDir": "",
            "Entrypoint": null,
            "OnBuild": null,
            "Labels": {
                "org.opencontainers.image.ref.name": "ubuntu",
                "org.opencontainers.image.version": "20.04"
            }
        },
        "Architecture": "amd64",
        "Os": "linux",
        "Size": 72812471,
        "GraphDriver": {
            "Data": {
                "MergedDir": "/var/lib/docker/overlay2/bf82f9ed685e183c6f04d217ad06a25be89eb9b552093fd0906fb4f688fc4866/merged",
                "UpperDir": "/var/lib/docker/overlay2/bf82f9ed685e183c6f04d217ad06a25be89eb9b552093fd0906fb4f688fc4866/diff",
                "WorkDir": "/var/lib/docker/overlay2/bf82f9ed685e183c6f04d217ad06a25be89eb9b552093fd0906fb4f688fc4866/work"
            },
            "Name": "overlay2"
        },
        "RootFS": {
            "Type": "layers",
            "Layers": [
                "sha256:fffe76c64ef2dee2d80a8bb3ad13d65d596d04a45510b1956a976a69215dae92"
            ]
        },
        "Metadata": {
            "LastTagTime": "0001-01-01T00:00:00Z"
        }
    }
]
```

Up 状態のコンテナに任意のコマンドを実行させることもできる。exec はコンテナを指定する。

```shell
wsluser@PC:~/prj/docker$ docker container ls
CONTAINER ID   IMAGE          COMMAND       CREATED       STATUS              PORTS     NAMES
7b6f8640f4f4   ubuntu:20.04   "/bin/bash"   2 hours ago   Up About a minute             crazy_shamir
 
wsluser@PC:~/prj/docker$ docker container stop 7b6f8640f4f4
7b6f8640f4f4
 
wsluser@PC:~/prj/docker$ docker container start 7b6f8640f4f4
7b6f8640f4f4
 
wsluser@PC:~/prj/docker$ docker container exec 7b6f8640f4f4 ls /home
docker.txt
 
wsluser@PC:~/prj/docker$ docker container exec -it 7b6f8640f4f4 bash
root@7b6f8640f4f4:/# ls home
docker.txt
```
 
```shell
wsluser@PC:~/prj/docker$ docker container run --name my-ubuntu ubuntu:20.04
 
wsluser@PC:~/prj/docker$ docker container ls -a
CONTAINER ID   IMAGE          COMMAND                  CREATED          STATUS                      PORTS     NAMES
720bc78330a9   ubuntu:20.04   "/bin/bash"              6 seconds ago    Exited (0) 5 seconds ago              my-ubuntu
20fb610608e8   ubuntu:20.04   "/bin/bash"              15 minutes ago   Exited (0) 15 minutes ago             quirky_heisenberg
31aeb0c0eb8b   ubuntu:20.04   "ls"                     16 minutes ago   Exited (0) 16 minutes ago             wizardly_meitner
9e98aa10b197   ubuntu:20.04   "ls"                     16 minutes ago   Exited (0) 16 minutes ago             angry_yonath
7b6f8640f4f4   ubuntu:20.04   "/bin/bash"              2 hours ago      Up 8 minutes                          crazy_shamir
78f65cf3b5f9   nginx          "/docker-entrypoint.…"   3 hours ago      Exited (0) 3 hours ago                gallant_elbakyan
3dd029b770a2   hello-world    "/hello"                 3 hours ago      Exited (0) 3 hours ago                quizzical_shaw
```
 
docker container prune で停止中のコンテナをすべて消せる。
 
```shell
wsluser@PC:~/prj/docker$ docker container ls -a
CONTAINER ID   IMAGE          COMMAND                  CREATED          STATUS                      PORTS     NAMES
720bc78330a9   ubuntu:20.04   "/bin/bash"              6 seconds ago    Exited (0) 5 seconds ago              my-ubuntu
20fb610608e8   ubuntu:20.04   "/bin/bash"              15 minutes ago   Exited (0) 15 minutes ago             quirky_heisenberg
31aeb0c0eb8b   ubuntu:20.04   "ls"                     16 minutes ago   Exited (0) 16 minutes ago             wizardly_meitner
9e98aa10b197   ubuntu:20.04   "ls"                     16 minutes ago   Exited (0) 16 minutes ago             angry_yonath
7b6f8640f4f4   ubuntu:20.04   "/bin/bash"              2 hours ago      Up 8 minutes                          crazy_shamir
78f65cf3b5f9   nginx          "/docker-entrypoint.…"   3 hours ago      Exited (0) 3 hours ago                gallant_elbakyan
3dd029b770a2   hello-world    "/hello"                 3 hours ago      Exited (0) 3 hours ago                quizzical_shaw
wsluser@PC:~/prj/docker$ docker container prune
WARNING! This will remove all stopped containers.
Are you sure you want to continue? [y/N] y
Deleted Containers:
720bc78330a96b4e0a9975d0968a17ce2889e7a3b10f88a132becf68a55e6fe2
20fb610608e83c34812c4192f577d01f6bc313ac3393e202fa166efbf05e4ee2
31aeb0c0eb8b568ac239d0c7fbba5354f9a43b6af545b956036da31f4137fb02
9e98aa10b197a46dd73edb9dc4096ca3a54caf3361fa5aa5868b7030bfd78f1e
78f65cf3b5f978599a1a4cca1072fc47de08b3d4315c8be124de4d95257d628c
3dd029b770a21e4c6a37d9c14c8706e823d1f60659e33547d1fc12325d6055e2
 
Total reclaimed space: 1.098kB
wsluser@PC:~/prj/docker$ docker container ls -a
CONTAINER ID   IMAGE          COMMAND       CREATED       STATUS          PORTS     NAMES
7b6f8640f4f4   ubuntu:20.04   "/bin/bash"   2 hours ago   Up 12 minutes             crazy_shamir
```
