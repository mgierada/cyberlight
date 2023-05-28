#!/bin/bash

# Install docker - see info here: https://docs.docker.com/engine/install/ubuntu/#install-using-the-convenience-script
curl -fsSL https://test.docker.com -o test-docker.sh
sudo sh test-docker.sh
# Start docker service
sudo service docker start
# Add ubuntu user to docker group
sudo usermod -a -G docker ubuntu
# Source the .bashrc file to apply changes or logout and login again
source ~/.bashrc
# Install AWS CLI
sudo apt install awscli
# Configure aws cli
aws configure
# Login to ECR
$(aws ecr get-login --no-include-email --region us-east-2)
# pull docker image from ECR
docker pull 458814646445.dkr.ecr.eu-north-1.amazonaws.com/rust_that_light:latest

