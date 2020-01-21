const createRole = name => caller =>
  caller.call("roles", "roles", "create_role", {
    name
  });

const assingRole = (roleName, agentAddress) => caller =>
  caller.call("roles", "roles", "assign_role", {
    name: roleName,
    agent_address: agentAddress
  });

module.exports = {
  createRole,
  assingRole
};
