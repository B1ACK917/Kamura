<template>
  <el-container class="kamura-viewer" style="height: 80vh">
    <el-scrollbar>
      <el-radio-group v-model="collapse.left" style="margin-bottom: 20px">
        <el-radio-button :value="true">
          <el-icon>
            <Minus/>
          </el-icon>
        </el-radio-button>
        <el-radio-button :value="false">
          <el-icon>
            <Plus/>
          </el-icon>
        </el-radio-button>
      </el-radio-group>
      <el-menu :collapse="collapse.left" :default-openeds="[]">
        <el-sub-menu index="1">
          <template #title>
            <el-icon>
              <Star/>
            </el-icon>
            <span>Arches</span>
          </template>
          <el-menu-item-group>
            <el-space direction="vertical">
              <el-space v-for="(arch, index) in this.arches" :key="index">
                <el-menu-item :index="`1-${index + 1}`" @click="fetchAndLoadCy(arch)">
                  {{ arch }}
                </el-menu-item>
                <el-button type="primary" :icon="Edit" circle @click="editArch(arch)"/>
              </el-space>
            </el-space>
          </el-menu-item-group>
        </el-sub-menu>

        <el-sub-menu index="3">
          <template #title>
            <el-icon>
              <Tickets/>
            </el-icon>
            <span>Tools</span>
          </template>
          <el-menu-item-group>
            <el-menu-item index="3-1">Undo</el-menu-item>
            <el-menu-item index="3-2">Redo</el-menu-item>
          </el-menu-item-group>
        </el-sub-menu>

        <el-sub-menu index="4">
          <template #title>
            <el-icon>
              <More/>
            </el-icon>
            <span>Options</span>
          </template>
          <el-menu-item-group>
            <el-menu-item index="4-1" @click="newArch">New</el-menu-item>
            <el-menu-item index="4-2" @click="duplicateArch">Duplicate</el-menu-item>
            <el-menu-item index="4-3" @click="saveArch()">Save</el-menu-item>
            <el-menu-item index="4-4" @click="resetArch">Reset</el-menu-item>
            <el-menu-item index="4-5" @click="removeArch">Delete</el-menu-item>
          </el-menu-item-group>
        </el-sub-menu>

      </el-menu>
    </el-scrollbar>

    <el-container>
      <el-main>
        <div ref="cyRef" style="width: 99%; height: 75vh; text-align: left" class="cyRef"></div>
      </el-main>
    </el-container>

    <el-scrollbar>
      <el-radio-group v-model="collapse.right" style="margin-bottom: 20px">
        <el-radio-button :value="true">
          <el-icon>
            <Minus/>
          </el-icon>
        </el-radio-button>
        <el-radio-button :value="false">
          <el-icon>
            <Plus/>
          </el-icon>
        </el-radio-button>
      </el-radio-group>
      <el-menu :collapse="collapse.right" :default-openeds="[]">

        <el-sub-menu index="1">
          <template #title>
            <el-icon>
              <FolderAdd/>
            </el-icon>
            <span>Add Unit</span>
          </template>
          <el-menu-item-group>
            <el-menu-item
                v-for="(_, unit, index) in this.eval.orderedUnits"
                :key="index"
                :index="`1-${index + 1}`"
                @click="addNewUnit(unit)"
            >
              {{ unit }}
            </el-menu-item>
          </el-menu-item-group>
        </el-sub-menu>

        <el-sub-menu index="2">
          <template #title>
            <el-icon>
              <Location/>
            </el-icon>
            <span>Go to Instance</span>
          </template>
          <el-menu-item-group>
            <el-menu-item
                v-for="(instance, index) in this.eval.instances"
                :key="index"
                :index="`2-${index + 1}`"
                @click="gotoInstance(instance)"
            >
              {{ instance }}
            </el-menu-item>
          </el-menu-item-group>
        </el-sub-menu>

      </el-menu>
    </el-scrollbar>
  </el-container>
  <el-container>
    <el-footer style="height: 10vh; margin-left: 5vw">
      <el-row :gutter="2">
        <el-col :span="8">
          <el-menu
              :default-active='"1"'
              class="kamura-footer-menu"
              mode="horizontal"
              :ellipsis="false"
          >
            <el-menu-item index="1" @click="this.mode='view';this.bindingStack=null">View Mode</el-menu-item>
            <el-menu-item index="2" @click="this.mode='bind';this.bindingStack=null">Bind Mode</el-menu-item>
            <el-menu-item index="3" @click="this.mode='bindFrom';this.bindingStack=null">Bind-From Mode
            </el-menu-item>
            <el-menu-item index="4" @click="this.mode='remove';this.bindingStack=null">Remove Mode</el-menu-item>
          </el-menu>
        </el-col>
        <el-col :span="4">
          <div>
            <el-text type="info" truncated>Selected Port</el-text>
          </div>
          <div>
            <el-text type="primary" truncated size="small">{{ bindingStack }}</el-text>
          </div>
        </el-col>
        <el-col :span="4">
          <div>
            <el-text type="info" truncated>Node-In</el-text>
          </div>
          <div
              v-for="(ele,index) in this.lastInformation.ins"
              :key="index">
            <el-text type="primary" truncated size="small">{{ ele }}</el-text>
          </div>
        </el-col>
        <el-col :span="4">
          <div>
            <el-text type="info" truncated>Node-Out</el-text>
          </div>
          <div
              v-for="(ele,index) in this.lastInformation.outs"
              :key="index">
            <el-text type="primary" truncated size="small">{{ ele }}</el-text>
          </div>
        </el-col>
        <el-col :span="4">
          <div>
            <el-text type="info" truncated>Edge Info</el-text>
          </div>
          <div>
            <el-text type="primary" truncated size="small">From: {{ lastInformation.from }}</el-text>
          </div>
          <div>
            <el-text type="primary" truncated size="small">To: {{ lastInformation.to }}</el-text>
          </div>
        </el-col>
      </el-row>

    </el-footer>
  </el-container>

  <el-dialog v-model="archEditDialogVisible" title="Edit Arch">
    <el-form :model="archEditDialogForm">
      <el-form-item label="Elements">
        <el-input
            v-model="archEditDialogForm.elements"
            style="width: 40vw; height: 50vh"
            :autosize="{ minRows: 1,maxRows:25 }"
            type="textarea"
            placeholder="Please input"
        />
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="archEditDialogVisible = false">Cancel</el-button>
        <el-button type="primary"
                   @click="archEditDialogVisible = false; saveArch(archEditDialogForm.target,archEditDialogForm.topology,JSON.parse(archEditDialogForm.elements))">
          Confirm
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>


<script>
import cytoscape from 'cytoscape';
import {options, styles} from '@/utils/layout';
import {kamuraEngineUrl, topologyTemplate, viewerDefaultVars} from "@/utils/consts";
import {
  addAUnit,
  checkBinding,
  commonFetchGet,
  commonFetchPost,
  parseArch,
  parseUnits,
  updateBinding
} from "@/utils/funcs";
import {ElMessage, ElMessageBox} from 'element-plus'
import {Edit} from "@element-plus/icons-vue";


export default {
  name: 'KamuraViewer',
  computed: {
    Edit() {
      return Edit
    }
  },
  props: ['updateSharedValues'],
  data() {
    return viewerDefaultVars;
  },
  mounted() {
    this.fetchArchesList();
    this.fetchUnitsList();
  },
  methods: {
    async fetchArchesList() {
      try {
        const data = await commonFetchGet(`${kamuraEngineUrl}/listArches`);
        this.arches = data.arches;
      } catch (error) {
        console.error("Error fetching arches:", error);
      }
    },
    async fetchUnitsList() {
      try {
        const data = await commonFetchGet(`${kamuraEngineUrl}/getUnits`);
        [this.raw.units, this.eval.orderedUnits] = parseUnits(data);
      } catch (error) {
        console.error("Error fetching units:", error);
      }
    },
    async fetchAndLoadCy(target) {
      this.selectedArch = target;
      this.$emit("updateSharedValues", target)
      await this.fetchArch(target, false);
      this.initCytoscape()
    },
    async fetchArch(target, reset) {
      try {
        const data = await commonFetchPost(`${kamuraEngineUrl}/getArchElements`, {target, reset});
        [this.cyElements, this.raw.topology, this.eval.instances] = parseArch(data);
      } catch (error) {
        console.error("Error fetching arch:", error);
      }
    },
    async saveArch(target = this.selectedArch, topology = this.raw.topology, elements = this.cyElements) {
      try {
        await commonFetchPost(`${kamuraEngineUrl}/saveArchElements`, {
          target,
          topology,
          elements
        });
        ElMessage({
          type: 'success',
          message: `Saved ${target} to Kamura-Engine`,
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
        await commonFetchPost(`${kamuraEngineUrl}/saveArchElements`, {
          target: value,
          topology: topologyTemplate,
          elements: []
        });
        await this.fetchAndLoadCy(value);
        ElMessage({type: 'success', message: `Successfully created new arch ${value}`,});
        await this.fetchArchesList();
      }).catch(() => {
        ElMessage({type: 'info', message: 'Canceled',})
      });
    },
    async duplicateArch() {
      if (this.selectedArch === null) {
        ElMessage({type: 'error', message: 'No selected arch',});
        return;
      }
      ElMessageBox.prompt(`Duplicate ${this.selectedArch}`, 'Tip', {
        confirmButtonText: 'Create',
        cancelButtonText: 'Cancel',
        inputPlaceholder: 'Arch Name'
      }).then(async ({value}) => {
        await commonFetchPost(`${kamuraEngineUrl}/saveArchElements`, {
          target: value,
          topology: this.raw.topology,
          elements: this.cyElements
        });
        await this.fetchAndLoadCy(value);
        ElMessage({type: 'success', message: `Duplicate ${this.selectedArch} as ${value}`,});
        await this.fetchArchesList();
      }).catch(() => {
        ElMessage({type: 'info', message: 'Canceled',})
      });
    },
    async removeArch() {
      if (!this.selectedArch) {
        ElMessage({
          type: 'error',
          message: 'No selected arch',
        })
      }
      ElMessageBox.confirm('Are you sure to delete this arch file?', 'Warning', {
        confirmButtonText: 'Confirm',
        cancelButtonText: 'Cancel',
        type: 'warning',
      }).then(async () => {
        try {
          await commonFetchPost(`${kamuraEngineUrl}/removeArch`, {target: this.selectedArch,});
          ElMessage({
            type: 'success',
            message: `Remove ${this.selectedArch} from Kamura-Engine`,
          })
          this.selectedArch = null;
          this.cy.destroy();
          this.cy = null;
          await this.fetchArchesList();
        } catch (error) {
          console.error("Error: ", error);
        }
      }).catch((err) => {
        console.error(err);
        ElMessage({
          type: 'info',
          message: 'Canceled',
        })
      });
    },
    async resetArch() {
      await this.fetchArch(this.selectedArch, true);
      this.initCytoscape();
    },
    initCytoscape() {
      this.createCytoscape();
      this.setListener();
    },
    createCytoscape() {
      this.cy = cytoscape({
        container: this.$refs.cyRef,
        elements: this.cyElements,
        style: styles.default,
        layout: options.preset
      });
    },
    addNewUnit(unitType) {
      if (this.selectedArch == null) {
        ElMessage({
          type: 'error',
          message: `No selected arch!`,
        });
        return;
      }
      ElMessageBox.prompt('Add a new unit instance', 'Tip', {
        confirmButtonText: 'Add',
        cancelButtonText: 'Cancel',
        inputValue: unitType
      }).then(({value}) => {
        [this.cyElements, this.raw.topology, this.eval.instances] = addAUnit(value, unitType, this.raw.units, this.raw.topology, this.cyElements, this.eval.instances);
        this.initCytoscape();
        ElMessage({
          type: 'success',
          message: `Add: ${value} as ${unitType}`,
        });
      }).catch((err) => {
        console.error(err);
        ElMessage({
          type: 'info',
          message: 'Add canceled',
        })
      });
    },
    gotoInstance(id) {
      this.cy.center(this.cy.getElementById(id));
    },
    async editArch(target) {
      this.archEditDialogVisible = true;
      try {
        const data = await commonFetchPost(`${kamuraEngineUrl}/getArchElements`, {target, reset: false});
        this.archEditDialogForm.target = target;
        this.archEditDialogForm.elements = JSON.stringify(parseArch(data)[0], null, 4);
        this.archEditDialogForm.topology = parseArch(data)[1];
      } catch (error) {
        console.error("Error fetching arch:", error);
      }
    },
    async setListener() {
      const self = this;
      this.cy.on('tap', 'node', async function (evt) {
        const node = evt.target;
        const nodeID = node.id();
        let valid = null, res = null, newEdge = null;
        switch (self.mode) {
          case 'remove':
            console.log(`${self.mode} not implemented`);
            break;
          case 'bind':
            [valid, res] = checkBinding(self.bindingStack, nodeID);
            if (!valid) {
              ElMessage.error(res);
              break;
            }
            [self.bindingStack, newEdge, self.raw.topology, self.cyElements] = updateBinding(self.bindingStack, nodeID, self.raw.topology, self.cyElements);
            if (newEdge.length !== 0) self.cy.add(newEdge);
            break;
          case 'bindFrom':
            [valid, res] = checkBinding(self.bindingStack, nodeID);
            if (!valid) {
              ElMessage.error(res);
              break;
            }
            [self.bindingStack, newEdge, self.raw.topology, self.cyElements] = updateBinding(self.bindingStack, nodeID, self.raw.topology, self.cyElements, false);
            if (newEdge.length !== 0) self.cy.add(newEdge);
            break;
          default:
            break;
        }
      });
      this.cy.on('tap', 'edge', function (evt) {
        console.log(self.cy.data());
        const edge = evt.target;
        const edgeID = edge.id();
        switch (self.mode) {
          case 'remove':
            console.log(`Remove mode: ${edgeID}`);
            self.cy.remove(self.cy.getElementById(edgeID));
            self.cyElements = self.cyElements.filter(item => item.data.id !== edgeID);
            const [source, target] = edgeID.split('-');
            self.raw.topology.binding = self.raw.topology.binding.filter(item => !(item.source === source && item.target === target));
            break;
          default:
            break;
        }
      });
      // this.cy.on('mouseover', 'node', async function (evt) {
      //   const node = evt.target;
      //   const nodeID = node.id();
      //   if (nodeID.includes('.')) {
      //     self.lastInformation.ins = node.incomers().filter(item => item.isNode()).map(item => item.id().replace('.ports', ''));
      //     self.lastInformation.outs = node.outgoers().filter(item => item.isNode()).map(item => item.id().replace('.ports', ''));
      //     self.lastInformation.name = nodeID.replace('.ports', '');
      //     self.lastInformation.inDegree = node.indegree();
      //     self.lastInformation.outDegree = node.outdegree();
      //   }
      // });
      // this.cy.on('mouseover', 'edge', async function (evt) {
      //   const edge = evt.target;
      //   self.lastInformation.from = edge.source().id().replace('.ports', '');
      //   self.lastInformation.to = edge.target().id().replace('.ports', '');
      // });
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
