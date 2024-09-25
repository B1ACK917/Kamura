<template>
  <el-container class="kamura-runner" style="height: 80vh">
    <el-scrollbar>
      <el-radio-group v-model="collapse" style="margin-bottom: 20px">
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
      <el-menu :collapse="collapse">
        <el-sub-menu index="1">
          <template #title>
            <el-icon>
              <DocumentAdd/>
            </el-icon>
            <span>Workloads</span>
          </template>
          <!-- Trace Subgroup -->
          <el-menu-item-group title="Trace">
            <el-menu-item
                v-for="(workload, index) in traceWorkloads"
                :key="'trace-' + index"
                @click="addTaskToRunner(workload[0], workload[1])"
            >
              {{ workload[0] }}
            </el-menu-item>
          </el-menu-item-group>
          <!-- ELF Subgroup -->
          <el-menu-item-group title="Elf">
            <el-menu-item
                v-for="(workload, index) in elfWorkloads"
                :key="'elf-' + index"
                @click="addTaskToRunner(workload[0], workload[1])"
            >
              {{ workload[0] }}
            </el-menu-item>
          </el-menu-item-group>
          <!-- ELSE Subgroup -->
          <el-menu-item-group title="Else">
            <el-menu-item
                v-for="(workload, index) in elseWorkloads"
                :key="'else-' + index"
                @click="addTaskToRunner(workload[0], workload[1])"
            >
              {{ workload[0] }}
            </el-menu-item>
          </el-menu-item-group>
        </el-sub-menu>
        <el-sub-menu index="2">
          <template #title>
            <el-icon>
              <Files/>
            </el-icon>
            <span>Runners</span>
          </template>
          <el-menu-item-group>
            <el-menu-item v-for="task in tasks" :key="task.uuid" @click="fetchTaskLog(task.uuid)">
              <el-icon :style="{ color: task.color }">
                <component :is="task.icon"/>
              </el-icon>
              {{ task.uuid }}
            </el-menu-item>
          </el-menu-item-group>
        </el-sub-menu>
      </el-menu>
    </el-scrollbar>

    <el-main>
      <div>
        <textarea id="log" readonly></textarea>
      </div>
      <el-space size="large">
        <el-descriptions>
          <el-descriptions-item label="Arch">
            <el-tag size="small">{{ arch }}</el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="Workload">
            <el-tag size="small">{{ workload }}</el-tag>
          </el-descriptions-item>
        </el-descriptions>
      </el-space>
    </el-main>
  </el-container>
</template>

<script>
import axios from 'axios';
import {kamuraEngineUrl} from "@/utils/consts";
import {commonFetchPost} from "@/utils/funcs";

export default {
  name: 'KamuraRunner',
  props: {
    sharedSelectedArch: {
      type: String, // Adjust the type based on the expected type of `sharedSelectedArch`
      default: null
    }
  },
  data() {
    return {
      collapse: false,
      tasks: [],  // Store tasks with uuid, color, icon
      traceWorkloads: [],
      elfWorkloads: [],
      elseWorkloads: [],
      wsList: [],
      selectedArch: null,
      logWS: null,
      arch: null,
      workload: null
    };
  },
  async created() {
    await this.fetchAllTasks();
    await this.fetchWorkloads();
    await this.startUpdateTaskStatuses();
  },
  beforeUnmount() {
    for (let ws of this.wsList) {
      ws.close();
    }
    if (this.logWS != null) this.logWS.close();
  },
  methods: {
    async fetchWorkloads() {
      try {
        const response = await axios.get('https://kamura-engine.malloc.fun/getValidWorkloads');
        if (response.data.success) {
          this.traceWorkloads = response.data.workloads.filter(workload => workload[1] === 'trace');
          this.elfWorkloads = response.data.workloads.filter(workload => workload[1] === 'elf');
          this.elseWorkloads = response.data.workloads.filter(workload => workload[1] === 'else');
        } else {
          console.error('Failed to fetch workloads');
        }
      } catch (error) {
        console.error('Error fetching workloads:', error);
      }
    },
    async fetchAllTasks() {
      try {
        const response = await axios.get(`${kamuraEngineUrl}/getAllTasks`);
        if (response.data.success) {
          const newTasks = response.data.tasks.filter(task => {
            return !this.tasks.some(existingTask => existingTask.uuid === task);
          });

          newTasks.forEach(task => {
            this.tasks.push({
              uuid: task,
              color: 'gray',
              icon: "SuccessFilled"
            });
          });
        }
      } catch (error) {
        console.error('Error fetching tasks:', error);
      }
    },
    async startUpdateTaskStatuses() {
      this.wsList = [];
      for (let task of this.tasks) {
        const ws = new WebSocket(`${kamuraEngineUrl}/ws/getTaskStatus/${task.uuid}`);
        ws.onmessage = (message) => {
          switch (message.data) {
            case "Succeed":
              task.color = "green";
              task.icon = "SuccessFilled";
              break;
            case "Failed":
              task.color = "red";
              task.icon = "CircleCloseFilled";
              break;
            case "Running":
              task.color = "yellow";
              task.icon = "InfoFilled";
              break;
            default:
              task.color = "gray";
              task.icon = "InfoFilled";
              break;
          }
        }
        this.wsList.push(ws);
      }
    },
    async fetchTaskLog(uuid) {
      if (this.logWS != null) this.logWS.close();
      this.fetchTaskInfo(uuid).then();
      this.logWS = new WebSocket(`${kamuraEngineUrl}/ws/getTaskLog/${uuid}`);
      this.logWS.onmessage = (message) => {
        let responseSpan = document.getElementById("log");
        responseSpan.innerHTML = message.data;
      }
    },
    async fetchTaskInfo(uuid) {
      try {
        const data = await commonFetchPost(`${kamuraEngineUrl}/getTaskInfo`, {target: uuid});
        this.arch = data.arch;
        this.workload = data.workload;
      } catch (error) {
        console.error("Error fetching log info:", error);
      }
    },
    async addTaskToRunner(workload, workloadType) {
      try {
        const response = await axios.post(`${kamuraEngineUrl}/addTask`, {
          arch: this.sharedSelectedArch,
          workload: workload,
          workload_type: workloadType
        });
        console.log('Response:', response.data);
      } catch (error) {
        console.error('Error:', error);
      }
      await this.fetchAllTasks();
      await this.startUpdateTaskStatuses();
    }
  }
}
</script>

<style scoped>
#log {
  width: 98%;
  height: 700px;
  margin: 0 auto;
  padding: 10px;
  text-align: left;
  border-radius: 4px;
  overflow-x: hidden;
  overflow-y: auto;
}
</style>
