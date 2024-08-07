export default {
  data() {
    return {
      encryptionTime: 0,
      decryptionTime: 0,
    };
  },
  methods: {
    timeOperation(operation: Function): number {
      const startTime = performance.now();
      operation();
      const endTime = performance.now();
      return endTime - startTime;
    },
  },
};
