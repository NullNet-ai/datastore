module.exports = {
  apps: [
    {
      name: 'crdt-store',
      cwd: './bin/store',
      script: 'cargo',
      args: 'run',
      interpreter: 'none',
      env: {
        RUST_BACKTRACE: '1',
        NODE_ENV: 'development'
      },
      env_production: {
        RUST_BACKTRACE: '0',
        NODE_ENV: 'production'
      },
      watch: false,
      instances: 1,
      exec_mode: 'fork',
      autorestart: true,
      max_restarts: 10,
      min_uptime: '10s',
      max_memory_restart: '1G',
      error_file: './logs/store-error.log',
      out_file: './logs/store-out.log',
      log_file: './logs/store-combined.log',
      time: true
    }
  ]
};