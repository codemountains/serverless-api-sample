# Severless API Sample

ビルドコマンド

```shell
docker image build -t rust-lambda-build -f Dockerfile.build .
```

```shell
docker container run --rm -v $PWD:/code -v $HOME/.cargo/registry:/root/.cargo/registry -v $HOME/.cargo/git:/root/.cargo/git rust-lambda-build
```

デプロイコマンド

```shell
sam package --template-file deploy.yaml --output-template-file packaged.yaml --s3-bucket severless-api-sample
```

```shell
sam deploy --template-file packaged.yaml --stack-name hello-api --capabilities CAPABILITY_IAM
```
