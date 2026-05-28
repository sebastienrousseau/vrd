# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Homebrew formula template for vrd. Populated by CI (or by
# hand per pkg/PUBLISH.md) from the v__VERSION__ release.
#
# Copy into sebastienrousseau/homebrew-tap as Formula/vrd.rb.

class Vrd < Formula
  desc "Lightweight, no_std-friendly random number generator for Rust"
  homepage "https://vrdlib.com"
  url "https://github.com/sebastienrousseau/vrd/archive/refs/tags/v__VERSION__.tar.gz"
  sha256 "__SHA256__"
  license any_of: ["Apache-2.0", "MIT"]
  head "https://github.com/sebastienrousseau/vrd.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args(path: ".")
  end

  test do
    # Smoke test the CLI demo binary.
    output = shell_output("#{bin}/vrd --help 2>&1", 0)
    assert_match(/Versatile Random Distributions/i, output)
  end
end
