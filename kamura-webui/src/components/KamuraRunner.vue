<template>
  <el-container class="kamura-runner" style="height: 80vh">
    <el-aside width="400px">
      <el-scrollbar>
        <el-menu>
          <el-sub-menu index="1">
            <template #title>
              Workloads
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
          </el-sub-menu>
          <el-sub-menu index="2">
            <template #title>
              Runners
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
    </el-aside>

    <el-main>
      <div>
        <textarea id="log" readonly></textarea>
      </div>
    </el-main>
  </el-container>
</template>

<script>
import axios from 'axios';
import {SuccessFilled, CircleCloseFilled, QuestionFilled} from '@element-plus/icons-vue';
import {kamura_engine_url} from "@/utils/consts";

export default {
  name: 'KamuraRunner',
  data() {
    return {
      tasks: [],  // Store tasks with uuid, color, icon
      traceWorkloads: [],
      elfWorkloads: [],
      wsList: []
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
  },
  methods: {
    async fetchWorkloads() {
      try {
        const response = await axios.get('https://kamura-engine.malloc.fun/getValidWorkloads');
        if (response.data.success) {
          this.traceWorkloads = response.data.workloads.filter(workload => workload[1] === 'trace');
          this.elfWorkloads = response.data.workloads.filter(workload => workload[1] === 'elf');
        } else {
          console.error('Failed to fetch workloads');
        }
      } catch (error) {
        console.error('Error fetching workloads:', error);
      }
    },
    async fetchAllTasks() {
      try {
        const response = await axios.get(`${kamura_engine_url}/getAllTasks`);
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
        const ws = new WebSocket(`${kamura_engine_url}/ws/getTaskStatus/${task.uuid}`);
        ws.onmessage = (message) => {
          switch (message.data) {
            case "Succeed":
              task.color = "green";
              task.icon = "SuccessFilled";
              break;
            case "Failed":
              task.color = "red";
              task.icon = "SuccessFilled";
              break;
            case "Running":
              task.color = "yellow";
              task.icon = "SuccessFilled";
              break;
            default:
              task.color = "gray";
              task.icon = "SuccessFilled";
              break;
          }
        }
        this.wsList.push(ws);
      }
    },
    async fetchTaskLog(uuid) {
      try {
        const response = await axios.post(`${kamura_engine_url}/getTaskLog`, {uuid});
        if (response.data.success) {
          let responseSpan = document.getElementById("log")
          responseSpan.innerHTML = response.data.message
        }
      } catch (error) {
        console.error('Error fetching task log:', error);
      }
    },
    async addTaskToRunner(workload, workloadType) {
      try {
        const response = await axios.post(`${kamura_engine_url}/addTask`, {
          arch: 'simple',
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
