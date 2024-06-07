# Local routing setup
Following, we are pulling the _dinghy-http-proxy_ in order to route all traffic as needed. The .docker domain will be resolved to the local machine.
And in our compose.yaml file we are setting the `VIRTUAL_HOST` environment variable to the domain we want to use.

Thus, our development domains are: `project-key.docker`.

To open a site locally our browser either needs `http` or a trailing `/`. Otherwise, it will search for the domain on the internet.
Https scheme is available as well as we are routing 443 per default.

## Mac

```shell
docker run -d --restart=always \
  -v /var/run/docker.sock:/tmp/docker.sock:ro \
  -v ~/.dinghy/certs:/etc/nginx/certs \
  -p 80:80 -p 443:443 -p 19322:19322/udp \
  -e CONTAINER_NAME=http-proxy \
  --name http-proxy \
  codekitchen/dinghy-http-proxy
```

```shell
sudo mkdir -pv /etc/resolver
sudo bash -c 'echo "nameserver 127.0.0.1" > /etc/resolver/docker'
sudo bash -c 'echo "port 19322" >> /etc/resolver/docker'
```

### Troubleshooting
`sudo lsof -i -n -P | grep "*:80"` should return a service listening (on TCP *:80) name _com.docker_

If it doesn't, you might check whether other services (as apache2) are running and blocking the local port 80.


## Linux

## Windows
