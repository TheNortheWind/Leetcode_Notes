# one pass hasmap solution

class Solution:
    def twoSum(self, nums, target):
        hashtable = {}
        n = len(nums)
        for i in range(n):
            compliment = target - nums[i]
            if compliment in hashtable:
                return [hashtable[compliment], i] # if solution found
            hashtable[nums[i]] = i # builds hashtable
            
        return [] # if not solution fount
Solution().twoSum([2,7,11,15],9)
