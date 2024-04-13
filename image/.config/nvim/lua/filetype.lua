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
