# Watch: 'rs_dev'
dev: build
  @cd rs_dev && cargo watch -x run

# ===================== #
# === BUILD SCRIPTS === #
# ===================== #

# Build: 'rs_dev' and all dependencies
build:
  @cargo build

# Build: 'rs_dev'
build-all:
  @cargo build

# Build: 'rs_dev'
build-dev:
  @cargo build -p r_dev

# Build: 'rs_logger'
build-logger:
  @cargo build -p rs_logger

# Build: 'rs_logger_macro'
build-logger-macro:
  @cargo build -p rs_logger_macro

# Build: 'rs_logger_errors'
build-logger-errors:
  @cargo build -p rs_logger_errors

# ====================== #
# === EXPAND SCRIPTS === #
# ====================== #

# Expand: All
expand: expand-logger-errors && expand-logger-macro expand-logger expand-dev

# Expand: 'rs_dev'
expand-dev:
  @cd rs_dev && cargo expand

# Expand: 'rs_logger'
expand-logger:
  @cd rs_logger && cargo expand

# Expand: 'rs_logger_macro'
expand-logger-macro:
  @cd rs_logger_macro && cargo expand

# Expand: 'rs_logger_errors'
expand-logger-errors:
  @cd rs_logger_errors && cargo expand

# Expand: Dev Logger
expand-dev-logger:
  @cd rs_dev && cargo expand logger

# =========================== #
# === MODULE TREE SCRIPTS === #
# =========================== #

# Module Tree: All
mods: mods-logger-errors mods-logger-macro mods-logger mods-dev

# Module Tree: 'rs_dev'
mods-dev:
  @cargo-modules generate tree -p rs_dev --types --traits --fns

# Module Tree: 'rs_logger'
mods-logger:
  @cargo-modules generate tree -p rs_logger --types --traits --fns

# Moduel Tree: 'rs_logger_macro'
mods-logger-macro:
  @cargo-modules generate tree -p rs_logger_macro --types --traits --fns

# Module Tree: 'rs_logger_errors'
mods-logger-errors:
  @cargo-modules generate tree -p rs_logger_errors --types --traits --fns
