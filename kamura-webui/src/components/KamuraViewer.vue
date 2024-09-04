<template>
  <el-container class="kamura-viewer" style="height: 80vh">
    <el-aside width="200px">
      <el-scrollbar>
        <el-menu :default-openeds="['1','2','3']">
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
              <el-menu-item index="2-1" @click="">View</el-menu-item>
              <el-menu-item index="2-2" @click="">Remove</el-menu-item>
              <el-menu-item index="2-4" @click="">Undo</el-menu-item>
              <el-menu-item index="2-5">Redo</el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>

          <el-sub-menu index="3">
            <template #title>
              Options
            </template>
            <el-menu-item-group>
              <el-menu-item index="3-1" @click="newArch">New</el-menu-item>
              <el-menu-item index="3-2" @click="saveArch">Save</el-menu-item>
              <el-menu-item index="3-3" @click="resetArch">Reset</el-menu-item>
              <el-menu-item index="3-4" @click="removeArch">Delete</el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>

        </el-menu>
      </el-scrollbar>
    </el-aside>

    <el-main>
      <div ref="cyRef" style="width: 99%; height: 75vh; text-align: left" class="cyRef"></div>
    </el-main>

    <el-aside width="200px">
      <el-scrollbar>
        <el-menu :default-openeds="['1']">

          <el-sub-menu index="1">
            <template #title>
              Add Unit
            </template>
            <el-menu-item-group>
              <el-menu-item
                  v-for="(_, unit, index) in this.units"
                  :key="index"
                  :index="`1-${index + 1}`"
                  @click="addNewUnit(unit)"
              >
                {{ unit }}
              </el-menu-item>
            </el-menu-item-group>
          </el-sub-menu>

        </el-menu>
      </el-scrollbar>
    </el-aside>

  </el-container>
</template>


<script>
import cytoscape from 'cytoscape';
import {options} from '@/utils/layout';
import {kamura_engine_url, topology_template} from "@/utils/consts";
import axios from "axios";
import {addAUnit} from "@/utils/funcs";
import {ElMessage, ElMessageBox} from 'element-plus'


export default {
  name: 'KamuraViewer',
  data() {
    return {
      arches: [],
      selectedArch: null,
      topology: null,
      units: null,
      cy: null,
      cyElements: [],
      removedNodes: [],
    };
  },
  mounted() {
    this.fetchArchesList();
    this.fetchUnitsList();
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
    async fetchUnitsList() {
      try {
        const response = await axios.get(`${kamura_engine_url}/getUnits`);
        const data = response.data;
        if (data.success) {
          const orderedUnits = Object.keys(data.units).sort().reduce(
              (obj, key) => {
                obj[key] = data.units[key];
                return obj;
              },
              {}
          );
          this.units = orderedUnits;
        } else {
          console.error("Failed to fetch units:", data.message);
        }
      } catch (error) {
        console.error("Error fetching units:", error);
      }
    },
    async fetchAndLoadCy(target) {
      this.selectedArch = target;
      await this.fetchArch(target, false);
      this.initCytoscape()
    },
    async fetchArch(target, reset) {
      try {
        const response = await axios.post(`${kamura_engine_url}/getArchElements`, {
          target,
          reset
        });
        const data = response.data;
        this.cyElements = data.elements;
        this.topology = data.topology;
      } catch (error) {
        console.error("Error fetching arch:", error);
      }
    },
    async saveArch() {
      try {
        await axios.post(`${kamura_engine_url}/saveArchElements`, {
          target: this.selectedArch,
          topology: this.topology,
          elements: this.cyElements
        });
        ElMessage({
          type: 'success',
          message: `Saved ${this.selectedArch} to Kamura-Engine`,
        })
      } catch (error) {
        console.error("Error saving arch:", error);
      }
    },
    async newArch() {
      ElMessageBox.prompt('Create a new arch', 'Tip', {
        confirmButtonText: 'Create',
        cancelButtonText: 'Cancel',
        inputPlaceholder: 'Arch Name'
      }).then(async ({value}) => {
        await axios.post(`${kamura_engine_url}/saveArchElements`, {
          target: value,
          topology: topology_template,
          elements: []
        });
        await this.fetchAndLoadCy(value);
        ElMessage({
          type: 'success',
          message: `Successfully created new arch ${value}`,
        });
        await this.fetchArchesList();
      }).catch(() => {
        ElMessage({
          type: 'info',
          message: 'Canceled',
        })
      });
    },
    async removeArch() {
      if (!this.selectedArch) {
        ElMessage({
          type: 'error',
          message: 'No selected arch',
        })
      }
      try {
        await axios.post(`${kamura_engine_url}/removeArch`, {
          target: this.selectedArch,
        });
        ElMessage({
          type: 'success',
          message: `Remove ${this.selectedArch} from Kamura-Engine`,
        })
        this.selectedArch = null;
      } catch (error) {
        console.error("Error: ", error);
      }
      await this.fetchArchesList();
    },
    async resetArch() {
      await this.fetchArch(this.selectedArch, true);
      this.initCytoscape();
    },
    initCytoscape() {
      this.cy = this.createCyto();
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
    addNewUnit(unitType) {
      ElMessageBox.prompt('Add a new unit instance', 'Tip', {
        confirmButtonText: 'Add',
        cancelButtonText: 'Cancel',
        inputValue: unitType
      }).then(({value}) => {
        ElMessage({
          type: 'success',
          message: `Add: ${value} as ${unitType}`,
        })
        this.cyElements = addAUnit(value, unitType, this.units, this.topology, this.cyElements);
        this.initCytoscape();
      }).catch(() => {
        ElMessage({
          type: 'info',
          message: 'Add canceled',
        })
      });
    }
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
