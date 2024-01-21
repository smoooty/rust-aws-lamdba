```bash
cargo lambda build --release --arm64 --output-format zip
```

```bash
AWS_PROFILE=<your_profile> terraform -chdir=infra plan
AWS_PROFILE=<your_profile> terraform -chdir=infra apply
```
