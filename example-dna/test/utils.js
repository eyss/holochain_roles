const createRole = caller => roleName =>
  caller.call("rolesTest", "example", "create_role", { role_name: roleName });

const assignRole = caller => (roleName, agentAddress) =>
  caller.call("rolesTest", "example", "assign_role", {
    role_name: roleName,
    agent_address: agentAddress
  });

const unassignRole = caller => (roleName, agentAddress) =>
  caller.call("rolesTest", "example", "create_role", {
    role_name: roleName,
    agent_address: agentAddress
  });

const getAllRoles = caller => () =>
  caller.call("rolesTest", "example", "get_all_roles", {});

const getRole = caller => roleName =>
  caller.call("rolesTest", "example", "get_role", { role_name: roleName });

const getAgentRoles = caller => agentAddress =>
  caller.call("rolesTest", "example", "get_agent_roles", {
    agent_address: agentAddress
  });

module.exports = {
  createRole,
  assignRole,
  unassignRole,
  getAgentRoles,
  getRole,
  getAllRoles
};
