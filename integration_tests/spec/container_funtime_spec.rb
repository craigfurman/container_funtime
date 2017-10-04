require 'open3'
require 'rspec'

RSpec.describe 'container funtime' do
  let(:project_root) { File.dirname(File.dirname(File.dirname(__FILE__))) }
  let(:funtime_bin) { File.join(project_root, 'target', 'debug', 'container_funtime') }

  it 'runs the user process, forwarding stdout' do
    stdout, stderr, status = Open3.capture3(
      funtime_bin, 'bash', '-c', 'echo stdout && echo stderr >&2 && exit 42',
      chdir: project_root
    )
    expect(status.exitstatus).to eq(42)
    expect(stdout).to eq("stdout\n")
    expect(stderr).to eq("stderr\n")
  end
end
