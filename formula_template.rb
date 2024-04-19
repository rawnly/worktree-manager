class Worktreemanager < Formula
  desc '{{description}}'
  homepage '{{homepage}}'
  url '{{repo}}/releases/download/{{version}}/{{bin}}.tar.gz'
  sha256 '{{shasum}}'
  version '{{version}}'

  def install
    bin.install '{{bin}}'
  end
end
