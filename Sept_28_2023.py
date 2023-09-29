class Solution(object):
    def sortArrayByParity(self,nums):
        self.numl = nums
        parity_nums = []
        limbo = []
        eyeglass = len(limbo)
        tick = True
        while tick == True:
            for i in self.numl:
                if i%2==0:
                    if 0 <= i and i <= 5000: #The constraints
                        parity_nums.append(i)
                    else:
                        break
                else:
                    if 0<= i and i <= 5000: #The constraints
                        limbo.append(i)
                    else:
                        break
            for y in limbo:
                parity_nums.append(y)
            tick = False
        return parity_nums
t = Solution()
t.sortArrayByParity([3,1,2,4])
