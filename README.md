<p align="center">
  <a href="https://load.network">
    <img src="https://gateway.load.rs/bundle/0x83cf4417880af0d2df56ce04ecfc108ea4ee940e8fb81400e31ab81571e28d21/0">
  </a>
</p>

## About
Load.Network provides an S3 implementation which enables developers to store files permanently in a decentralized manner by using the common AWS S3 Patterns with minimal change.

## Installation
Load.Network is compatible with the S3 SDKs, because of this, you are able to use existing libraries.

### NodeJS
To install the official S3 library in NodeJS, run the following command

```shell
$ yarn add @aws-sdk/client-s3
```

#### Initialization

In order to initialize the S3 client connected to Load Network, you can do the following:

```typescript
import { S3Client } from "@aws-sdk/client-s3";

const accessKeyId = process.env.LOAD_ACCESS_KEY;
const secretAccessKey = ""; // It's meant to be empty

const s3Client = new S3Client({
    region: "eu-west-2", // Required -- current supported region
    endpoint: "https://s3.load.rs", // Load.Network S3 endpoint
    credentials: {
        accessKeyId,
        secretAccessKey,
    },
    forcePathStyle: true, // Required
});
```
- `process.env.LOAD_ACCESS_KEY`: Contains your private service key in [cloud.load.network](https://cloud.load.network). 
  - It looks similar to `load_acc_*******`
- `https://s3.load.rs` is the endpoint for the S3 interface provided by Load Network.
- `forcePathStyle` set to `true` is *always* necessary.

## License
This project is licensed under the [MIT License](./LICENSE)