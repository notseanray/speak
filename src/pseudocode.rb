BEST = (
	VALUE INDEX,
	MEGA INDEX 
)

for VALUE in VALUES:

	for VALUECHUNK in VALUE.chunk:

		for INPUTCHUNK in INPUT.chunk:

			for x, MEGAVALUE in MEGA:
		
				for MCHUNK in MEGAVALUE.chunk:

								#	KEYSUM		INPUTSUM
								# ---------- - ----------
								#  VALUESUM		VALUESUM

					CALCULATION = MCHUNK.sum - ( INPUTCHUNK.sum / VALUECHUNK.sum )

					if CALCULATION < MEGA[x].chunks[BEST.1].sum - ()
end