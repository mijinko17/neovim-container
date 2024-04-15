return {
  'nvim-telescope/telescope.nvim',
  dependencies = { 'nvim-lua/plenary.nvim', 'BurntSushi/ripgrep' },
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
