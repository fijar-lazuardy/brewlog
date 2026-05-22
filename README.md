## Development Setup

Install [mise](https://mise.jdx.dev/) then set up the development environment:

```bash
sudo apt update
sudo apt install -y clang mold pkg-config libssl-dev \
    libatk1.0-0 libatk-bridge2.0-0 libcups2 libdrm2 libxkbcommon0 \
    libxcomposite1 libxdamage1 libxrandr2 libgbm1 libpango-1.0-0 \
    libcairo2 libasound2t64 libnss3 libxshmfence1
mise trust
mise install              # Install all dev tools
mise run install-e2e      # Install Chrome for Testing + ChromeDriver
prek install              # Install git hooks
```

## Contributing

```bash
prek run -av              # Run all lints, tests, formatters
mise run fmt              # Format all files
mise run check            # Full CI validation (fmt + lint + test)
mise run test             # Run all tests
cargo build               # Build
```

See [CLAUDE.md](CLAUDE.md) for architecture, code patterns, and development conventions.

## License

[Apache License 2.0](LICENSE)
