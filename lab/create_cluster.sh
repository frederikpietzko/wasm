#!/bin/sh

kind create cluster \
  --config ./lab/ClusterConfig.yaml

CONTAINER_IP=$(docker inspect lab-control-plane --format '{{.NetworkSettings.Networks.kind.IPAddress}}')
kubectl config set-cluster kind-lab --server=https://${CONTAINER_IP}:6443
