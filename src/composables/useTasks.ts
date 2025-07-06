import { invoke } from "@tauri-apps/api/core";
import { Ref, ref } from "vue";

type DateTimeString = string;

export interface ITask {
  name: string;
  desc?: string;
  timestamp: DateTimeString;
}

export const useTasks = () => {
  const tasks = ref([]) as Ref<Array<ITask>>;
  const displayNewTaskModal = ref(false);
  const taskCreationError = ref(undefined) as Ref<string | undefined>;

  const loadTasks = async () => {
    try {
      tasks.value = await invoke("get_tasks");
      console.log("tasks loaded");

      console.log(tasks.value);
    } catch (e) {
      console.log(e);
    }
  };

  const addTask = async (e: ITask) => {
    if (!e.name || !e.timestamp) {
      console.log("missing required attributes");
      taskCreationError.value = "Missing required attributes!";
      return;
    }
    try {
      console.log("Adding task:");
      console.log(e);
      const task = {
        name: e.name,
        desc: e.desc,
        timestamp: new Date(e.timestamp).toISOString()
      };

      await invoke("create_task", task);
      console.log("task created!");

      tasks.value.push(task);

      toggleCreateModal();
    } catch (e) {
      console.log(e);
    }
  };

  const toggleCreateModal = () => {
    taskCreationError.value = undefined;
    displayNewTaskModal.value = !displayNewTaskModal.value;
  };

  return {
    tasks,
    displayNewTaskModal,
    taskCreationError,
    loadTasks,
    addTask,
    toggleCreateModal
  };
};
