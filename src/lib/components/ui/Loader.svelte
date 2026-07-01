<div class="fire">
	<div class="fire-left">
		<div class="main-fire"></div>
		<div class="particle-fire"></div>
	</div>
	<div class="fire-center">
		<div class="main-fire"></div>
		<div class="particle-fire"></div>
	</div>
	<div class="fire-right">
		<div class="main-fire"></div>
		<div class="particle-fire"></div>
	</div>
	<div class="fire-bottom">
		<div class="main-fire"></div>
	</div>
</div>

<style>
	.fire {
		position: absolute;
		top: calc(50% - 25px);
		left: calc(50% - 25px);
		width: 50px;
		height: 50px;
		background-color: transparent;
		margin-left: auto;
		margin-right: auto;
	}
	@keyframes scaleUpDown {
		0%,
		100% {
			transform: scaleY(1) scaleX(1);
		}

		50%,
		90% {
			transform: scaleY(1.1);
		}

		75% {
			transform: scaleY(0.95);
		}

		80% {
			transform: scaleX(0.95);
		}
	}

	@keyframes shake {
		0%,
		100% {
			transform: skewX(0) scale(1);
		}

		50% {
			transform: skewX(5deg) scale(0.9);
		}
	}

	@keyframes particleUp {
		0% {
			opacity: 0;
		}

		20% {
			opacity: 1;
		}

		80% {
			opacity: 1;
		}

		100% {
			opacity: 0;
			top: -100%;
			transform: scale(0.5);
		}
	}

	@keyframes glow {
		0%,
		100% {
			background-color: var(--color, #48BF91);
		}

		50% {
			background-color: var(--color, #48BF91);
		}
	}

	.fire {
		position: absolute;
		top: calc(50% - 25px);
		left: calc(50% - 25px);
		width: 50px; /* Reduced size by 50% */
		height: 50px; /* Reduced size by 50% */
		background-color: transparent;
		margin-left: auto;
		margin-right: auto;
		transform: scale(var(--scale));
	}

	.fire-center {
		position: absolute;
		height: 100%;
		width: 100%;
		animation: scaleUpDown calc(3s / var(--speed)) ease-out infinite both;
	}

	.fire-center .main-fire {
		position: absolute;
		width: 100%;
		height: 100%;
		background-image: radial-gradient(farthest-corner at 10px 0, var(--color, #48BF91) 0%, var(--color, #48BF91) 95%);
		transform: scaleX(0.8) rotate(45deg);
		border-radius: 0 40% 60% 40%;
		filter: drop-shadow(0 0 10px var(--color, #48BF91));
	}

	.fire-center .particle-fire {
		position: absolute;
		top: 60%;
		left: 45%;
		width: 10px;
		height: 10px;
		background-color: var(--color, #48BF91);
		border-radius: 50%;
		filter: drop-shadow(0 0 10px #d43322);
		animation: particleUp calc(2s / var(--speed)) ease-out infinite both;
	}

	.fire-right {
		height: 100%;
		width: 100%;
		position: absolute;
		animation: shake calc(2s / var(--speed)) ease-out infinite both;
	}

	.fire-right .main-fire {
		position: absolute;
		top: 15%;
		right: -25%;
		width: 80%;
		height: 80%;
		background-color: var(--color, #48BF91);
		transform: scaleX(0.8) rotate(45deg);
		border-radius: 0 40% 60% 40%;
		filter: drop-shadow(0 0 10px var(--color, #48BF91));
	}

	.fire-right .particle-fire {
		position: absolute;
		top: 45%;
		left: 50%;
		width: 15px;
		height: 15px;
		background-color: var(--color, #48BF91);
		transform: scaleX(0.8) rotate(45deg);
		border-radius: 50%;
		filter: drop-shadow(0 0 10px var(--color, #48BF91));
		animation: particleUp calc(2s / var(--speed)) ease-out infinite both;
	}

	.fire-left {
		position: absolute;
		height: 100%;
		width: 100%;
		animation: shake calc(3s / var(--speed)) ease-out infinite both;
	}

	.fire-left .main-fire {
		position: absolute;
		top: 15%;
		left: -20%;
		width: 80%;
		height: 80%;
		background-color: var(--color, #48BF91);
		transform: scaleX(0.8) rotate(45deg);
		border-radius: 0 40% 60% 40%;
		filter: drop-shadow(0 0 10px var(--color, #48BF91));
	}

	.fire-left .particle-fire {
		position: absolute;
		top: 10%;
		left: 20%;
		width: 10%;
		height: 10%;
		background-color: var(--color, #48BF91);
		border-radius: 50%;
		filter: drop-shadow(0 0 10px #d43322);
		animation: particleUp calc(3s / var(--speed)) ease-out infinite both;
	}

	.fire-bottom .main-fire {
		position: absolute;
		top: 30%;
		left: 20%;
		width: 75%;
		height: 75%;
		background-color: var(--color, #48BF91);
		transform: scaleX(0.8) rotate(45deg);
		border-radius: 0 40% 100% 40%;
		filter: blur(10px);
		animation: glow calc(2s / var(--speed)) ease-out infinite both;
	}
</style>
