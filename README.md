## build

```bash
cargo lambda build --release --arm64 --output-format zip
```

## deploy

```bash
AWS_PROFILE=<your_profile> terraform -chdir=infra plan
AWS_PROFILE=<your_profile> terraform -chdir=infra apply
```

## dev

```bash
cargo lambda watch
cargo lambda invoke rust-aws-lambda --data-ascii '{}'
```
