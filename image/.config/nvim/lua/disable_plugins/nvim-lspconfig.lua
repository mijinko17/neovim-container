return {
  "neovim/nvim-lspconfig",
  dependencies = { "williamboman/mason.nvim" },
  config = function()
    require("mason-lspconfig").setup_handlers {
      function(server_name)
        require("lspconfig")[server_name].setup {}
      end,
    }
  end
}

