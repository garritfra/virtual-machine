const gates = {};

gates.and = (a, b) => !!a && !!b;
gates.or = (a, b) => !!a || !!b;
gates.not = a => !a;

module.exports = gates;
