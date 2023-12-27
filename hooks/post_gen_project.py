# cat post_gen_project.py
import os
import shutil

print(os.getcwd())

def remove(filepath):
    if os.path.isfile(filepath):
        os.remove(filepath)
    elif os.path.isdir(filepath):
        shutil.rmtree(filepath)

print({{ cookiecutter.use_helix }}, {{ cookiecutter.use_vscode }}, {{ cookiecutter.include_sample_code }} )

# use_helix = '' == 'y'
# use_vscode = '' == 'y'

if not {{ cookiecutter.use_helix }}:
    remove('.helix')

if not {{ cookiecutter.use_vscode }}:
    remove('.vscode')
