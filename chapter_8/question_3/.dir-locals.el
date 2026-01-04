;; Tell rustic/LSP to link the local Cargo.toml
((rustic-mode
  (lsp-rust-analyzer-linked-projects . ["Cargo.toml"])
  ;; Optional: ensure LSP starts automatically
  (eval . (lsp-deferred))))
