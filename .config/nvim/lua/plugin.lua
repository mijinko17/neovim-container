local ensure_packer = function()
  local fn = vim.fn
  local install_path = fn.stdpath('data') .. '/site/pack/packer/start/packer.nvim'
  if fn.empty(fn.glob(install_path)) > 0 then
    fn.system({ 'git', 'clone', '--depth', '1', 'https://github.com/wbthomason/packer.nvim', install_path })
    vim.cmd [[packadd packer.nvim]]
    return true
  end
  return false
end

vim.api.nvim_create_autocmd('FileType', {
  pattern = '*',
  callback = function(args)
    if args.match == 'lua' then
      vim.bo.expandtab   = true
      vim.bo.shiftwidth  = 2
      vim.bo.softtabstop = 2
      vim.bo.tabstop     = 2
    end
  end
})

local packer_bootstrap = ensure_packer()
vim.cmd [[autocmd BufWritePre <buffer> lua vim.lsp.buf.format()]]

return require('packer').startup(function(use)
  use 'wbthomason/packer.nvim'
  use {
    "williamboman/mason.nvim",
  }
  use {
    "williamboman/mason-lspconfig.nvim",
    config = function()
      --	    require("mason").setup()
      --	    require("mason-registry").refresh()
      --	    require("mason-lspconfig").setup{
      --		    ensure_installed = { "lua_ls"            },
      --	    }
    end
  }
  use {
    "neovim/nvim-lspconfig",
    config = function()
      require("mason").setup()
      require("mason-registry").refresh()
      require("mason-lspconfig").setup {
      }
      vim.api.nvim_create_user_command('Upper', 'LspInstall lua_ls', { nargs = 0 })

      require("mason-lspconfig").setup_handlers {
        function(server_name) -- default handler (optional)
          require("lspconfig")[server_name].setup {}
        end,
      }
    end
  }
  if packer_bootstrap then
    require('packer').sync()
  end
end)
