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

return require('packer').startup(function(use)
  use 'wbthomason/packer.nvim'
  use "williamboman/mason.nvim"
  use "williamboman/mason-lspconfig.nvim"
  use {
    "neovim/nvim-lspconfig",
    config = function()
      require("mason").setup()
      require("mason-registry").refresh()
      require("mason-lspconfig").setup()
      require("mason-lspconfig").setup_handlers {
        function(server_name)
          require("lspconfig")[server_name].setup {}
        end,
      }
      vim.cmd [[autocmd BufWritePre <buffer> lua vim.lsp.buf.format()]]
    end
  }
  use {
    'hrsh7th/nvim-cmp',
    requires = {
      'hrsh7th/cmp-path',
      'hrsh7th/cmp-nvim-lsp',
    },
    config = function()
      vim.opt.completeopt = { 'menu', 'menuone', 'noselect' }
      require 'cmp'.setup { sources = { { name = 'path' }, { name = 'nvim_lsp' } } }
    end
  }
  use {
    'nvim-telescope/telescope.nvim',
    requires = { { 'nvim-lua/plenary.nvim' }, { 'BurntSushi/ripgrep' } },
    config = function()
      local builtin = require('telescope.builtin')
      vim.keymap.set('n', '<leader>p', function() builtin.find_files({ hidden = true }) end, {})
      vim.keymap.set('n', '<leader>f', builtin.live_grep, {})
      require('telescope').setup {
        defaults = {
          file_ignore_patterns = {
            ".git"
          }
        }
      }
    end
  }
  use {
    'kristijanhusak/vim-hybrid-material',
    config = function()
      vim.cmd 'colorscheme hybrid_material'
    end,
  }
  if packer_bootstrap then
    require('packer').sync()
  end
end)
