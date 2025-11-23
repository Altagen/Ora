# Ora Registry Examples

This directory shows how to set up and use Ora package registries.

## Two Registry Modes

Ora supports two ways to distribute .repo files:

### 1. Git Registry (Multiple Packages)

**Use when:** You want to maintain a collection of packages in a Git repository.

**Setup:**
```bash
# Create a Git repository
mkdir my-registry
cd my-registry
git init

# Create the ora-registry directory (convention)
mkdir ora-registry

# Add .repo files
cp path/to/package1.repo ora-registry/
cp path/to/package2.repo ora-registry/

# Commit and push
git add .
git commit -m "Initial registry"
git push origin main
```

**Usage:**
```bash
# Users add your registry
ora registry add my-registry https://github.com/user/my-registry.git

# Install packages
ora install package1
ora install package2
```

**Directory Structure:**
```
my-registry/
├── ora-registry/        # ← Convention: put .repo files here
│   ├── package1.repo
│   ├── package2.repo
│   └── package3.repo
├── README.md
└── LICENSE
```

---

### 2. Direct URL Registry (Single Package)

**Use when:** You want to serve a single .repo file without Git infrastructure.

**Setup:**
```bash
# Just serve a .repo file via your web server (nginx, Apache, CDN, etc.)
# No Git repository needed!

# Example: Serve via nginx
/var/www/html/ora/windsurf.repo
```

**nginx configuration:**
```nginx
location /ora/ {
    root /var/www/html;
    add_header Content-Type text/plain;
    add_header Access-Control-Allow-Origin *;
}
```

**Usage:**
```bash
# Users add your .repo file directly
ora registry add windsurf https://windsurf.com/ora/windsurf.repo

# Install the package
ora install windsurf
```

**Perfect for:**
- Software vendors distributing their own app
- Simple single-package distribution
- Static file hosting (GitHub Pages, Cloudflare, etc.)
- No Git maintenance overhead

---

## Convention: `/ora-registry/` Directory

For Git repositories, Ora looks for .repo files in the `/ora-registry/` directory by default.

**Why `/ora-registry/` and not `/packages/`?**
- Avoids conflicts with existing `/packages/` directories
- Clear indication this is an Ora registry
- Reduces security risks (no path traversal issues)

**Can I use a different path?**
Yes! Configure it when adding the registry:
```bash
ora registry add my-registry https://github.com/user/project.git --path custom-path
```

---

## Embedded Registry in Your Project

You can add an Ora registry to an existing project:

```
your-project/
├── src/              # Your project code
├── docs/
├── Cargo.toml
├── ora-registry/     # Ora registry embedded in your project
│   └── your-project.repo
└── README.md
```

**Benefits:**
- Self-distributing: your project includes its own .repo file
- Versioned with your code
- Easy for contributors to install from source

**Usage:**
```bash
ora registry add your-project https://github.com/user/your-project.git
ora install your-project
```

---

## Example Registry

The `ora-registry/` directory in this folder shows an example registry with:
- prometheus.repo
- ripgrep.repo

**To use this example:**
```bash
# Clone this repository
git clone https://github.com/ora-pm/ora.git
cd ora/examples/registry

# Use as local registry
ora install prometheus --repo ./ora-registry/prometheus.repo
```

---

## Multiple Versions Support

For packages with multiple release channels (stable/beta/next), create separate .repo files:

**Git Registry:**
```
ora-registry/
├── windsurf.repo       # Stable channel
└── windsurf-next.repo  # Beta channel
```

**Direct URL:**
```bash
# Serve two files
ora registry add windsurf https://windsurf.com/ora/windsurf.repo
ora registry add windsurf-next https://windsurf.com/ora/windsurf-next.repo
```

Each .repo file is treated as a separate package/registry.

---

## Security Considerations

### Git Registries
- **Integrity:** Git commits can be GPG-signed
- **Trust:** Users trust the repository owner
- **Updates:** `ora registry update` pulls latest changes

### Direct URL Registries
- **Integrity:** HTTPS ensures transport security
- **Trust:** Users trust the domain owner
- **Updates:** Fetched fresh on each `ora install`

**Best Practices:**
1. Always use HTTPS for direct URLs
2. Sign Git commits for Git registries
3. Provide checksums in .repo files when possible
4. Document security expectations in README

---

## Creating Your Own Registry

### Option 1: GitHub Repository

```bash
# 1. Create repository
gh repo create my-ora-registry --public

# 2. Clone and set up
git clone https://github.com/user/my-ora-registry.git
cd my-ora-registry
mkdir ora-registry

# 3. Add packages
cp ../packages/*.repo ora-registry/

# 4. Publish
git add .
git commit -m "Add initial packages"
git push
```

### Option 2: Static Web Hosting

```bash
# 1. Create .repo file
vi windsurf.repo

# 2. Upload to web server / CDN
scp windsurf.repo server:/var/www/html/ora/

# Or use GitHub Pages, Cloudflare Pages, etc.
```

---

## Registry Discovery

Users can discover your registry by:
1. **Documentation:** Link in your project README
2. **Website:** Dedicated page listing available packages
3. **Community:** Share in Ora community registry list (coming soon)

Example README section:
```markdown
## Installation via Ora

```bash
ora registry add my-app https://myapp.com/ora/my-app.repo
ora install my-app
```
```

---

## Examples

### Dedicated Git Registry
See the `ora-registry/` directory for a simple example.

### Embedded Registry
Check out how Windman project includes its own registry:
```
https://github.com/user/windman/ora-registry/windman.repo
```

### Direct URL
Windsurf serves .repo files directly:
```
https://windsurf.com/ora/windsurf.repo (example)
```

---

## See Also

- [Creating .repo files](../../docs/CREATING_REPO_FILES.md)
- [Provider examples](../providers/)
- [Security best practices](../../ROADMAP/SECURITY.md)
