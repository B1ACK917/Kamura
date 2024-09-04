import yaml from 'js-yaml';

export function parseYaml(yamlText) {
  try {
    return yaml.load(yamlText);
  } catch (e) {
    console.error(e);
    return null;
  }
}

function SpiltModulePort(bindingTopology) {
  for (let i = 0; i < bindingTopology.length; i++) {
    bindingTopology[i] = bindingTopology[i].replace(".ports", "");
    bindingTopology[i] = bindingTopology[i].split('.');
  }
  return bindingTopology;
}

function classify(bindingTopology) {
  const modules = {}
  for (let i = 0; i < bindingTopology.length; i++) {
    const [moduleName, portName] = bindingTopology[i];
    if (moduleName in modules) {
      modules[moduleName].add(portName);
    }
    else {
      modules[moduleName] = new Set();
      modules[moduleName].add(portName);
    }
  }
  for (const moduleName in modules) {
    modules[moduleName] = [...modules[moduleName]];
  }
  const ordered = Object.keys(modules).sort().reduce(
    (obj, key) => {
      obj[key] = modules[key];
      return obj;
    },
    {}
  );
  return ordered;
}

function drawModules(modules, nodeGap = [10, 10], moduleGap = [10, 10]) {
  const elements = [];
  const numModules = Object.keys(modules).length;
  const cols = Math.ceil(Math.sqrt(numModules));
  let [x, y] = [0, 0];
  const [nodeGapX, nodeGapY] = nodeGap;
  const [moduleGapX, moduleGapY] = moduleGap;
  let c = 0;

  for (const moduleName in modules) {
    elements.push({ data: { id: moduleName, label: moduleName }, classes: 'subgraph' });
    const numPorts = modules[moduleName].length;
    const numPerLine = Math.ceil(numPorts / 2);

    for (let i = 0; i < numPorts; i++) {
      const portName = modules[moduleName][i];
      const posX = x + nodeGapX * (i % numPerLine);
      const posY = y + nodeGapY * Math.floor(i / numPerLine);
      console.log(`${moduleName}-${portName}: (${posX},${posY})`)
      const data = {
        data: {
          id: `${moduleName}-${portName}`,
          label: portName,
          parent: moduleName
        },
        position: { x: posX, y: posY }
      };
      // console.log(data);
      elements.push(data);
    }
    x += nodeGapX * numPerLine + moduleGapX;
    if (++c == cols) {
      c = x = 0;
      y += nodeGapY + moduleGapY;
    }
  }
  return elements;
}

export function parseBindings(bindingTopology) {
  bindingTopology = SpiltModulePort(bindingTopology);
  const modules = classify(bindingTopology);
  const elements = drawModules(modules, [200, 100], [300, 300]);

  // for (let i = 0; i < bindingTopology.length - 1; i += 2) {
  //   const [sourceModule, sourcePortName] = bindingTopology[i];
  //   const [targetModule, targetPortName] = bindingTopology[i + 1];
  //   elements.push({ data: { id: `${sourceModule}-${sourcePortName}-${targetModule}-${targetPortName}`, source: `${sourceModule}-${sourcePortName}`, target: `${targetModule}-${targetPortName}` } });
  // }
  return elements;
}