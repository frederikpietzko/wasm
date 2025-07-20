#!/bin/sh

kind create cluster \
  --name lab

CONTAINER_IP=$(docker inspect lab-control-plane --format '{{.NetworkSettings.Networks.kind.IPAddress}}')
kubectl config set-cluster kind-lab --server=https://${CONTAINER_IP}:6443
