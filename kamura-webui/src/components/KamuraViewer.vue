<template>
  <el-container class="kamura-viewer" style="height: 80vh">
    <el-aside width="200px">
      <el-scrollbar>
        <el-menu :default-openeds="['1','2']">
          <el-sub-menu index="1">
            <template #title>
              Arches
            </template>
            <el-menu-item-group>
              <el-menu-item
                  v-for="(arch, index) in this.arches"
                  :key="index"
                  :index="`1-${index + 1}`"
                  @click="fetchAndLoadCy(arch)"
              >
                {{ arch }}
              </el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>

          <el-sub-menu index="2">
            <template #title>
              Tools
            </template>
            <el-menu-item-group>
              <el-menu-item index="2-1" @click="setMode('view')">View</el-menu-item>
              <el-menu-item index="2-2" @click="setMode('remove')">Remove</el-menu-item>
              <el-menu-item index="2-3">Add</el-menu-item>
              <el-menu-item index="2-4" @click="restore">Undo</el-menu-item>
              <el-menu-item index="2-5">Redo</el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>

          <el-sub-menu index="3">
            <template #title>
              Options
            </template>
            <el-menu-item-group>
              <el-menu-item index="3-1" @click="">New</el-menu-item>
              <el-menu-item index="3-2" @click="">Save</el-menu-item>
              <el-menu-item index="3-3" @click="">Reset</el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>

        </el-menu>
      </el-scrollbar>
    </el-aside>

    <el-main>
      <div ref="cyRef" style="width: 80%; height: 75vh; text-align: left" class="cyRef"></div>
    </el-main>

  </el-container>
</template>


<script>
import cytoscape from 'cytoscape';
import {options} from '@/utils/layout';
import {kamura_engine_url} from "@/utils/consts";
import axios from "axios";


export default {
  name: 'KamuraViewer',
  data() {
    return {
      arches: [],
      cy: null,
      cyElements: [],
      removedNodes: [],
    };
  },
  mounted() {
    this.fetchArchesList();
  },
  methods: {
    async fetchArchesList() {
      try {
        const response = await axios.get(`${kamura_engine_url}/listArches`);
        const data = response.data;
        if (data.success) {
          this.arches = data.arches;
        } else {
          console.error("Failed to fetch arches:", data.message);
        }
      } catch (error) {
        console.error("Error fetching arches:", error);
      }
    },
    async fetchAndLoadCy(target) {
      // const response = await fetch('/simple.yaml');
      // const yamlText = await response.text();
      // const data = parseYaml(yamlText);
      // if (data) {
      //   const bindingTopology = data['top.extension.topo_extensions']['binding_topology'];
      //   const t = parseBindings(bindingTopology);
      //   this.cyElements = t;
      //   console.log(t);
      //   this.initCytoscape();
      // }

      await this.fetchArch(target);
      this.initCytoscape()
    },
    async fetchArch(target) {
      try {
        const response = await axios.post(`${kamura_engine_url}/getArchElements`, {
          target
        });
        const data = response.data;
        this.cyElements = data.elements;
      } catch (error) {
        console.error("Error fetching arches:", error);
      }
    },
    initCytoscape() {
      this.cy = this.createCyto();

      this.setMode('view');
    },
    createCyto() {
      return cytoscape({
        container: this.$refs.cyRef,
        elements: this.cyElements,
        style: [
          {
            selector: 'node',
            style: {
              'background-color': '#666',
              'label': 'data(label)',
              'text-valign': 'center',
              'color': '#000',
              'width': '40px',
              'height': '40px',
            }
          },
          {
            selector: '.subgraph',
            style: {
              'background-color': '#ddd',
              'shape': 'rectangle',
              'padding': '10px',
              'border-color': '#333',
              'border-width': '2px'
            }
          },
          {
            selector: 'edge',
            style: {
              'line-color': '#111',
              'target-arrow-color': '#000',
              'target-arrow-shape': 'triangle'
            }
          }
        ],
        layout: options.preset
      });
    },
    setMode(mode) {
      console.log(this.cyElements);
      switch (mode) {
        case 'remove':
          this.cy.$('node').on('tap', (e) => {
            const ele = e.target;
            this.removedNodes.push(ele.remove());
          });
          break;
        case 'view':
          this.cy.$('node').on('tap', (e) => {
            e
          });
          break;
      }
    },
    restore() {
      let top = this.removedNodes.pop();
      if (top) {
        top.restore();
      }
    },
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.cyRef {
  border-style: solid;
  border-width: thin;
  border-color: gray;
}
</style>
