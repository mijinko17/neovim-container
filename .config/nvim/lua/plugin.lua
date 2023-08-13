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
  use 'L3MON4D3/LuaSnip'
  use 'saadparwaiz1/cmp_luasnip'
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
      vim.cmd [[autocmd BufWritePre * lua vim.lsp.buf.format()]]
      vim.keymap.set({ 'n', 'v' }, '<leader>rn', function() vim.lsp.buf.rename() end, {})
      vim.keymap.set({ 'n' }, '<leader>ca', function() vim.lsp.buf.code_action() end, {})
      vim.keymap.set('n', 'K', function() vim.lsp.buf.hover() end, {})
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
      local cmp = require 'cmp'
      require 'cmp'.setup({
        snippet = {
          expand = function(args)
            require('luasnip').lsp_expand(args.body)
          end,
        },
        mapping = cmp.mapping.preset.insert({
          ["<C-p>"] = cmp.mapping.select_prev_item(),
          ["<C-n>"] = cmp.mapping.select_next_item(),
          ['<C-l>'] = cmp.mapping.complete(),
          ['<C-e>'] = cmp.mapping.abort(),
          ["<CR>"] = cmp.mapping.confirm { select = true },
        }),
        sources = { { name = 'path' }, { name = 'nvim_lsp' } }
      })
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
  use {
    'ojroques/nvim-osc52',
    config = function()
      local function copy(lines, _)
        require('osc52').copy(table.concat(lines, '\n'))
      end

      local function paste()
        return { vim.fn.split(vim.fn.getreg(''), '\n'), vim.fn.getregtype('') }
      end
      vim.g.clipboard = {
        name = 'osc52',
        copy = { ['+'] = copy, ['*'] = copy },
        paste = { ['+'] = paste, ['*'] = paste },
      }
    end,
  }
  use({
    "jose-elias-alvarez/null-ls.nvim",
    requires = { "nvim-lua/plenary.nvim" },
    config = function()
      local null_ls = require("null-ls")
      null_ls.setup()
    end
  })
  use({
    "jay-babu/mason-null-ls.nvim",
    config = function()
      require("mason-null-ls").setup({
        handlers = {},
      })
    end
  })
  if packer_bootstrap then
    require('packer').sync()
  end
end)
