#!/bin/sh

CONFIG_FILE="${1:-./lab/ClusterConfig.yaml}"

kind create cluster \
  --config "${CONFIG_FILE}"

CONTAINER_IP=$(docker inspect lab-control-plane --format '{{.NetworkSettings.Networks.kind.IPAddress}}')
kubectl config set-cluster kind-lab --server=https://${CONTAINER_IP}:6444