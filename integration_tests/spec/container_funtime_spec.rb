require 'open3'
require 'rspec'

# rubocop:disable BlockLength
RSpec.describe 'container funtime' do
  let(:project_root) { File.dirname(File.dirname(File.dirname(__FILE__))) }
  let(:funtime_bin) { File.join(project_root, 'target', 'debug', 'container_funtime') }
  let(:alpine_rootfs) { ENV.fetch('CTR_TESTS_ALPINE_ROOTFS') }

  def run_containerised_process(*argv)
    Open3.capture3(funtime_bin, '--rootfs', alpine_rootfs, *argv, chdir: project_root)
  end

  it 'runs the user process, forwarding stdout' do
    stdout, stderr, status = run_containerised_process(
      '/bin/ash', '-c', 'echo stdout && echo stderr >&2 && exit 42'
    )
    expect(status.exitstatus).to eq(42)
    expect(stdout).to eq("stdout\n")
    expect(stderr).to eq("stderr\n")
  end

  it 'runs the user process in a UTS namespace' do
    stdout, stderr, status = run_containerised_process(
      '/bin/ash', '-c', '/bin/hostname newhostname && /bin/hostname'
    )
    expect(stdout).to eq("newhostname\n"), stdout + stderr
    expect(status.exitstatus).to eq(0)
  end

  it 'runs the user process in a specified rootfs directory' do
    stdout, stderr, status = run_containerised_process(
      '/bin/cat', '/etc/os-release'
    )
    expect(stdout).to include('NAME="Alpine Linux"'), stdout + stderr
    expect(status.exitstatus).to eq(0)
  end

  it 'unmounts the old rootfs, leaving only / and /proc' do
    stdout, stderr, status = run_containerised_process(
      '/bin/cat', '/proc/self/mounts'
    )
    expect(stdout).to include('/ '), stdout + stderr
    expect(stdout).to include('/proc '), stdout + stderr
    expect(stdout).to_not include('/oldrootfs'), stdout + stderr
    expect(status.exitstatus).to eq(0)
  end
end
