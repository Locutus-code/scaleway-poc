#!/bin/bash

docker build -f Dockerfile.be -t scaleway-be:latest .
docker tag -t scaleway-be:latest rg.fr-par.scw.cloud/funcscwbackendzz96jbax/scaleway-be:latest
docker push rg.fr-par.scw.cloud/funcscwbackendzz96jbax/scaleway-be:latest
