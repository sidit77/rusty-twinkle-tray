#!/usr/bin/env python3
"""
Simple validation script for the Build Release workflow.
This script validates that the workflow YAML is syntactically correct
and contains all required components.
"""

import yaml
import sys
from pathlib import Path

def validate_workflow():
    """Validate the Build Release workflow configuration."""
    workflow_path = Path(__file__).parent.parent / "workflows" / "build-release.yml"
    
    if not workflow_path.exists():
        print(f"❌ Workflow file not found: {workflow_path}")
        return False
    
    try:
        with open(workflow_path, 'r') as f:
            workflow = yaml.safe_load(f)
    except yaml.YAMLError as e:
        print(f"❌ Invalid YAML syntax: {e}")
        return False
    
    # Validate required top-level keys
    required_keys = ['name', 'on', 'jobs']
    for key in required_keys:
        if key not in workflow:
            print(f"❌ Missing required key: {key}")
            return False
    
    # Validate trigger configuration
    if 'release' not in workflow['on']:
        print("❌ Workflow should trigger on release")
        return False
    
    if workflow['on']['release']['types'] != ['created']:
        print("❌ Workflow should trigger on release created")
        return False
    
    # Validate job configuration
    jobs = workflow['jobs']
    if not jobs:
        print("❌ No jobs defined")
        return False
    
    job = list(jobs.values())[0]
    
    # Check if it runs on Windows
    if job.get('runs-on') != 'windows-latest':
        print("❌ Job should run on windows-latest")
        return False
    
    # Check for required permissions
    permissions = job.get('permissions', {})
    required_permissions = ['contents', 'id-token', 'attestations']
    for perm in required_permissions:
        if perm not in permissions:
            print(f"❌ Missing required permission: {perm}")
            return False
    
    # Check for essential steps
    steps = job.get('steps', [])
    step_names = [step.get('name', '') for step in steps]
    
    required_steps = [
        'Checkout code',
        'Setup Rust', 
        'Build release',
        'Generate artifact attestation',
        'Upload release asset'
    ]
    
    for required_step in required_steps:
        if not any(required_step in name for name in step_names):
            print(f"❌ Missing required step: {required_step}")
            return False
    
    print("✅ Build Release workflow validation passed!")
    print("📝 Workflow features:")
    print("  - Triggers on release creation")
    print("  - Builds on Windows with Rust")
    print("  - Includes artifact attestation")
    print("  - Uploads executable to release")
    return True

if __name__ == "__main__":
    success = validate_workflow()
    sys.exit(0 if success else 1)