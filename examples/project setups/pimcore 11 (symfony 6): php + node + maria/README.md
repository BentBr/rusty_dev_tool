# Pimcore 11 setup with symfony 6

Following services are used:
- php8.2 + messenger service (for async tasks)
- node 20
- nginx
- redis
- maria 8.0
- mailcatcher

This setup could be used as a default symfony 6 setup as well like for shopware 6 or other symfony 6 projects.

Missing files to be included:
- webpack.config.js (if you want to use webpack encore for asset management)
- package.json (if you want to use npm for asset management)
- composer.json (if you want to use composer for php packages)