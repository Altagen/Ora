# Ora Package Examples

This directory contains examples of .repo files organized by provider type and registry configurations.

## Directory Structure

```
examples/
├── providers/          # Examples by provider type
│   ├── github-releases/
│   ├── webpage-scraping/
│   ├── direct-url/
│   ├── gitlab-releases/
│   └── custom-api/
└── registry/           # Example registry configuration
    └── ora-registry/   # Registry with multiple packages
```

## Provider Examples

### [GitHub Releases](providers/github-releases/)

Examples of packages using GitHub releases as the source:
- **prometheus** - Monitoring system with checksums
- **ripgrep** - Fast grep alternative

### [Webpage Scraping](providers/webpage-scraping/)

Examples of packages that scrape download URLs from HTML pages:
- **windsurf** - Windsurf IDE (stable channel)
- **windsurf-next** - Windsurf IDE (beta channel)

### [Direct URL](providers/direct-url/)

Examples of packages with direct download URLs:
- **simple-tool** - Basic example with platform-specific URLs

### [GitLab Releases](providers/gitlab-releases/)

Examples of packages using GitLab releases (coming soon)

### [Custom API](providers/custom-api/)

Examples of packages using custom APIs for version discovery (coming soon)

## Registry Example

The [registry/](registry/) directory shows how to organize multiple .repo files in a registry format.

### Two Registry Modes

**1. Git Registry (for collections)**
```bash
# Host multiple packages in a Git repository
ora registry add my-registry https://github.com/user/my-registry.git
```

Your repository structure:
```
my-registry/
└── ora-registry/        # Convention: put .repo files here
    ├── package1.repo
    ├── package2.repo
    └── package3.repo
```

**2. Direct URL Registry (for single packages)**
```bash
# Serve a single .repo file via HTTPS
ora registry add windsurf https://windsurf.com/ora/windsurf.repo
```

Just serve the .repo file as a static file on your web server.

## Quick Start

### Using a local .repo file

```bash
# Install from a local .repo file
ora install package --repo ./examples/providers/github-releases/prometheus.repo
```

### Creating your own .repo file

1. Choose the appropriate provider type
2. Copy an example from the matching provider directory
3. Modify it for your package
4. Test it with `ora install --repo ./your-package.repo`

## Documentation

- [Creating .repo files](../docs/CREATING_REPO_FILES.md)
- [Provider types](../docs/PROVIDERS.md) (coming soon)
- [Registry setup](./registry/README.md)

## Contributing Examples

Have a useful .repo file to share? Please submit a PR with:
- The .repo file in the appropriate provider directory
- A brief description in this README
- Any special notes about the package
