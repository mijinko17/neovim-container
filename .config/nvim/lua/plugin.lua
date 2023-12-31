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

local packer_bootstrap = ensure_packer()

return require('packer').startup(function(use)
  use 'wbthomason/packer.nvim'
  use({
    "Pocco81/auto-save.nvim",
    config = function()
      require("auto-save").setup {}
    end,
  })
  use {
    "williamboman/mason.nvim",
    config = function()
      require("mason").setup()
      vim.keymap.set({ 'n' }, '<leader>fm', function() vim.lsp.buf.format() end, {})
      vim.keymap.set({ 'n', 'v' }, '<leader>rn', function() vim.lsp.buf.rename() end, {})
      vim.keymap.set({ 'n' }, '<leader>ca', function() vim.lsp.buf.code_action() end, {})
      vim.keymap.set('n', 'K', function() vim.lsp.buf.hover() end, {})
    end
  }
  use {
    "williamboman/mason-lspconfig.nvim",
    after = 'mason.nvim',
    config = function()
      require("mason-lspconfig").setup()
    end
  }
  use {
    after = 'mason-lspconfig.nvim',
    "neovim/nvim-lspconfig",
    config = function()
      require("mason-lspconfig").setup_handlers {
        function(server_name)
          require("lspconfig")[server_name].setup {}
        end,
      }
    end
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
  use 'L3MON4D3/LuaSnip'
  use 'saadparwaiz1/cmp_luasnip'
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
      vim.keymap.set('n', '<leader>ff', builtin.live_grep, {})
      require('telescope').setup {
        defaults = {
          file_ignore_patterns = {
            "^.git/"
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
    "lewis6991/gitsigns.nvim",
    config = function()
      local gitsigns = require('gitsigns')
      require('gitsigns').setup {
        signs                        = {
          add          = { text = '│' },
          change       = { text = '│' },
          delete       = { text = '_' },
          topdelete    = { text = '‾' },
          changedelete = { text = '~' },
          untracked    = { text = '┆' },
        },
        signcolumn                   = true,  -- Toggle with `:Gitsigns toggle_signs`
        numhl                        = false, -- Toggle with `:Gitsigns toggle_numhl`
        linehl                       = false, -- Toggle with `:Gitsigns toggle_linehl`
        word_diff                    = false, -- Toggle with `:Gitsigns toggle_word_diff`
        watch_gitdir                 = {
          follow_files = true
        },
        attach_to_untracked          = true,
        current_line_blame           = false, -- Toggle with `:Gitsigns toggle_current_line_blame`
        current_line_blame_opts      = {
          virt_text = true,
          virt_text_pos = 'eol', -- 'eol' | 'overlay' | 'right_align'
          delay = 1000,
          ignore_whitespace = false,
        },
        current_line_blame_formatter = '<author>, <author_time:%Y-%m-%d> - <summary>',
        sign_priority                = 6,
        update_debounce              = 100,
        status_formatter             = nil,   -- Use default
        max_file_length              = 40000, -- Disable if file is longer than this (in lines)
        preview_config               = {
          -- Options passed to nvim_open_win
          border = 'single',
          style = 'minimal',
          relative = 'cursor',
          row = 0,
          col = 1
        },
        yadm                         = {
          enable = false
        },
      }
      vim.keymap.set({ 'n' }, '<leader>vr', function() gitsigns.reset_hunk() end, {})
    end
  })
  use({
    "sindrets/diffview.nvim",
    requires = { { 'nvim-tree/nvim-web-devicons' } },
    config = function()
      vim.keymap.set({ 'n' }, '<leader>vd', ':DiffviewOpen<CR>', {})
      vim.keymap.set({ 'n' }, '<leader>vdc', ':DiffviewClose<CR>', {})
      require("diffview").setup({
        -- use_icons = false
      })
    end
  })
  use({
    "NeogitOrg/neogit",
    after = 'diffview.nvim',
    config = function()
      local neogit = require('neogit')
      neogit.setup {}
    end
  })
  if packer_bootstrap then
    require('packer').sync()
  end
end)
