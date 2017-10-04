require 'open3'
require 'rspec'

RSpec.describe 'container funtime' do
  let(:project_root) { File.dirname(File.dirname(File.dirname(__FILE__))) }

  it 'says hello' do
    stdout, _, status = Open3.capture3('cargo', 'run', chdir: project_root)
    expect(status).to eq(0)
    expect(stdout).to eq("Hello world!\n")
  end
end
